use core::ops::{Index, IndexMut};

use crate::float::Float;
use crate::program::SignalRef;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignalState {
  Off,
  Updated,
  Consumed,
}

#[derive(Debug, Clone)]
pub struct Signal<F: Float> {
  value: F,
  state: SignalState,
}

impl<F: Float> Default for Signal<F> {
  fn default() -> Self {
    Signal {
      value: F::zero(),
      state: SignalState::Updated,
    }
  }
}

impl<F: Float> Signal<F> {
  pub fn new(initial_value: F) -> Self {
    Signal {
      value: initial_value,
      state: SignalState::Updated,
    }
  }

  pub fn set(&mut self, value: F) {
    self.state = if self.value != value {
      SignalState::Updated
    } else {
      self.state
    };
    self.value = value;
  }

  pub fn consume(&mut self) -> F {
    if self.state == SignalState::Updated {
      self.state = SignalState::Consumed
    }
    self.value
  }

  pub fn get(&self) -> F {
    self.value
  }

  pub fn state(&self) -> SignalState {
    self.state
  }

  pub fn reset(&mut self) {
    self.state = SignalState::Updated;
  }

  pub fn update_state(&mut self) {
    self.state = match self.state {
      SignalState::Updated => SignalState::Updated,
      SignalState::Consumed => SignalState::Off,
      SignalState::Off => SignalState::Off,
    }
  }

  pub fn if_updated<G>(&mut self, f: G)
  where
    G: FnOnce(F) -> (),
  {
    match self.state {
      SignalState::Updated | SignalState::Consumed => f(self.consume()),
      SignalState::Off => {}
    }
  }
}

//#[macro_export]
//macro_rules! if_signal_updated {
//    ( $signal:expr, $o:expr, $setter:expr ) => {
//      {
//        let s = $signal;
//        if s.is_updated() {
//          $setter($o, s.get())
//        }
//      }
//    };
//}

pub(crate) struct SignalBus<'a, F: Float> {
  signals: &'a mut [Signal<F>],
}

impl<'a, F: Float> SignalBus<'a, F> {
  pub fn new(signals: &'a mut [Signal<F>]) -> Self {
    SignalBus { signals }
  }

  pub fn reset(&mut self) {
    for signal in self.signals.iter_mut() {
      signal.reset();
    }
  }

  pub fn update(&mut self) {
    for signal in self.signals.iter_mut() {
      signal.update_state();
    }
  }
}

impl<'a, F: Float, S: Into<SignalRef>> Index<S> for SignalBus<'a, F> {
  type Output = Signal<F>;
  fn index(&self, index: S) -> &Self::Output {
    &self.signals[index.into().0]
  }
}

impl<'a, F: Float, S: Into<SignalRef>> IndexMut<S> for SignalBus<'a, F> {
  fn index_mut(&mut self, index: S) -> &mut Self::Output {
    &mut self.signals[index.into().0]
  }
}

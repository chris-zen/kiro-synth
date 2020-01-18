use num_traits::Float;

use crate::program::SignalRef;
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Signal<F: Float> {
  value: F,
  updated: bool,
}

impl<F: Float> Default for Signal<F> {
  fn default() -> Self {
    Signal {
      value: F::zero(),
      updated: true,
    }
  }
}

impl<F: Float> Signal<F> {
  pub fn set(&mut self, value: F) {
    self.updated = true;
    self.value = value;
  }

  pub fn get(&self) -> F {
    self.value
  }

  pub fn is_updated(&self) -> bool {
    self.updated
  }

  pub fn reset_update(&mut self) {
    self.updated = false;
  }

  pub fn if_updated<G>(&self, f: G) where G: FnOnce(F) -> () {
    if self.updated {
      f(self.value)
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
  signals: &'a mut [Signal<F>]
}

impl<'a, F: Float> SignalBus<'a, F> {
  pub fn new(signals: &'a mut [Signal<F>]) -> Self {
    SignalBus { signals }
  }
}

impl<'a, F: Float> Index<&SignalRef> for SignalBus<'a, F> {
  type Output = Signal<F>;
  fn index(&self, index: &SignalRef) -> &Self::Output {
    &self.signals[index.0]
  }
}

impl<'a, F: Float> IndexMut<&SignalRef> for SignalBus<'a, F> {
  fn index_mut(&mut self, index: &SignalRef) -> &mut Self::Output {
    &mut self.signals[index.0]
  }
}

impl<'a, F: Float> Index<SignalRef> for SignalBus<'a, F> {
  type Output = Signal<F>;
  fn index(&self, index: SignalRef) -> &Self::Output {
    &self.signals[index.0]
  }
}

impl<'a, F: Float> IndexMut<SignalRef> for SignalBus<'a, F> {
  fn index_mut(&mut self, index: SignalRef) -> &mut Self::Output {
    &mut self.signals[index.0]
  }
}

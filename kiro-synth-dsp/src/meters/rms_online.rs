use generic_array::{ArrayLength, GenericArray};

use crate::float::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct RmsOnline<F, N: ArrayLength<F>> {
  pub squares: GenericArray<F, N>,
  pub head: usize,
}

impl<F: Float, N: ArrayLength<F>> Default for RmsOnline<F, N> {
  fn default() -> Self {
    RmsOnline {
      squares: GenericArray::default(),
      head: 0,
    }
  }
}

impl<F: Float + Default, N: ArrayLength<F>> RmsOnline<F, N> {
  pub fn reset(&mut self) {
    self.squares = GenericArray::default();
    self.head = 0;
  }

  pub fn process(&mut self, signal: F) {
    let square = signal * signal;
    self.squares[self.head] = square;
    self.head = (self.head + 1) % N::to_usize();
  }

  pub fn get(&self) -> F {
    let sum_of_squares = self.squares.iter().fold(F::zero(), |acc, v| acc + *v);
    (sum_of_squares / F::val(N::to_usize())).sqrt()
  }
}

#[cfg(test)]
mod test {
  #![allow(clippy::float_cmp)]

  use super::RmsOnline;
  use generic_array::{typenum::consts, GenericArray};

  const SIGNALS: [f32; 6] = [1.0, 2.0, 5.0, 4.0, 3.0, 1.0];

  #[test]
  fn test_default() {
    let rms = RmsOnline::<f32, consts::U4>::default();
    assert_eq!(
      rms,
      RmsOnline {
        squares: GenericArray::default(),
        head: 0,
      }
    );
  }

  #[test]
  fn test_reset() {
    let mut rms = RmsOnline::<f32, consts::U4>::default();
    SIGNALS.iter().for_each(|v| rms.process(*v));
    rms.reset();
    assert_eq!(rms.squares.as_slice(), &[0.0, 0.0, 0.0, 0.0]);
    assert_eq!(rms.head, 0);
  }

  #[test]
  fn test_process() {
    let mut rms = RmsOnline::<f32, consts::U4>::default();
    SIGNALS.iter().for_each(|v| rms.process(*v));
    assert_eq!(rms.squares.as_slice(), &[9.0, 1.0, 25.0, 16.0]);
    assert_eq!(rms.head, 2);
  }

  #[test]
  fn test_get() {
    let mut rms = RmsOnline::<f32, consts::U4>::default();
    SIGNALS.iter().for_each(|v| rms.process(*v));
    assert_eq!(rms.get(), 3.5707142);
  }
}

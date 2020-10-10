use crate::float::Float;
use crate::funcs::concave_transforms::concave_inverted_transform;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub struct Exponential;

impl Default for Exponential {
  fn default() -> Self {
    Exponential
  }
}

impl Exponential {
  pub fn new() -> Self {
    Exponential
  }
}

impl<F: Float> Waveform<F> for Exponential {
  fn initial_modulo(&self) -> F {
    F::zero()
  }

  fn generate(&mut self, modulo: F, _phase_inc: F) -> F {
    concave_inverted_transform(modulo)
  }
}

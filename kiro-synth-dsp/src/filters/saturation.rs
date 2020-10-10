use crate::float::Float;

#[derive(Debug)]
pub struct Saturation<F: Float> {
  enabled: bool,
  value: F,
}

impl<F: Float> Saturation<F> {
  pub fn new(enabled: bool) -> Self {
    Saturation {
      enabled,
      value: F::one(),
    }
  }

  pub fn saturate(&self, input: F) -> F {
    if self.enabled {
      (self.value * input).tanh()
    } else {
      input
    }
  }
}

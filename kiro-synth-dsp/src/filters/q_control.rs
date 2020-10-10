use crate::float::Float;

#[derive(Debug)]
pub struct QControl<F: Float> {
  scale: F,
  offset: F,
  value: F,
  invalidated: bool,
}

impl<F: Float> QControl<F> {
  pub fn new(min: F, max: F, value: F) -> Self {
    QControl {
      scale: max - min,
      offset: min,
      value,
      invalidated: true,
    }
  }

  pub fn is_invalidated(&self) -> bool {
    self.invalidated
  }

  pub fn set_value(&mut self, value: F) {
    self.invalidated = true;
    self.value = value.mul_add(self.scale, self.offset);
  }

  pub fn get_scaled_value(&mut self) -> F {
    self.invalidated = false;
    self.value
  }

  pub fn default_q() -> F {
    F::val(0.707)
  }
}

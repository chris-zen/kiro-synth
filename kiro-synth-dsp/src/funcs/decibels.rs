use crate::float::Float;

pub struct Decibels<F>(F);

impl<F: Float> Decibels<F> {
  pub fn new(db: F) -> Self {
    Decibels(db)
  }

  pub fn from_amplitude(amp: F) -> Decibels<F> {
    Decibels(F::val(20.0) * amp.abs().log10())
  }

  pub fn to_amplitude(&self) -> F {
    F::val(10.0).powf(self.0 / F::val(20.0))
  }

  pub fn value(&self) -> F {
    self.0
  }
}

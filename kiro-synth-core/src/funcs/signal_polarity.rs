use crate::float::Float;

pub fn unipolar_to_bipolar<F: Float>(signal: F) -> F {
  F::val(2.0) * signal - F::one()
}

pub fn bipolar_to_unipolar<F: Float>(signal: F) -> F {
  let half = F::val(0.5);
  half * signal + half
}

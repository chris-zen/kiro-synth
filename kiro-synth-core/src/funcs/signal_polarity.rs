use crate::float::Float;

pub fn unipolar_to_bipolar<F: Float>(signal: F) -> F {
  F::val(2.0) * signal - F::one()
}

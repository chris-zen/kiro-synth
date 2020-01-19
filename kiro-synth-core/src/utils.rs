use crate::float::Float;

pub fn unipolar_to_bipolar<F: Float>(signal: F) -> F {
  F::from(2.0).unwrap() * signal - F::one()
}

use crate::float::Float;

const CONVEX_LIMIT: f64 = 0.00398107;
const CONCAVE_LIMIT: f64 = 0.99601893;

/// calculates the concaveTransform of the input
/// value is a unipolar value to convert [0, 1]
pub fn concave_transform<F: Float>(value: F) -> F {
  if value >= F::val(CONCAVE_LIMIT) {
    F::one()
  } else {
    -(F::val(5.0) / F::val(12.0)) * (F::one() - value).log10()
  }
}

/// calculates the concaveInvertedTransform of the input
/// value is a unipolar value to convert [0, 1]
pub fn concave_inverted_transform<F: Float>(value: F) -> F {
  if value <= F::val(CONVEX_LIMIT) {
    F::one()
  } else {
    -(F::val(5.0) / F::val(12.0)) * value.log10()
  }
}

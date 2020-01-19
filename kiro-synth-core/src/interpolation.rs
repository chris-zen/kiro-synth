use crate::float::Float;

pub fn linear_interpolation<F: Float>(x1: F, x2: F, y1: F, y2: F, x: F) -> F {
  let denom = x2 - x1;

  // should not ever happen
  if denom.is_zero() {
    y1
  } else {
    // calculate decimal position of x
    let dx = (x - x1) / denom;

    // use weighted sum method of interpolating
    dx * y2 + (F::one() - dx) * y1
  }
}

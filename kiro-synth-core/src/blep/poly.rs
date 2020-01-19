use crate::float::Float;

pub struct PolyBLEP;

impl PolyBLEP {
  /// 2-point polynomial BLEP
  pub fn residual<F: Float>(modulo: F, phase_inc: F, height: F, rising_edge: bool) -> F {
    // --- return value
    // --- LEFT side of discontinuity
    //	   -1 < t < 0
    let residual = if modulo > F::one() - phase_inc {
      // --- calculate distance
      let t = (modulo - F::one()) / phase_inc;

      // --- calculate residual
      height * (t * t + F::from(2.0).unwrap() * t + F::one())
    }
    // --- RIGHT side of discontinuity
    //     0 <= t < 1
    else if modulo < phase_inc {
      // --- calculate distance
      let t = modulo / phase_inc;

      // --- calculate residual
      height * (F::from(2.0).unwrap() * t - t * t - F::one())
    } else {
      F::zero()
    };

    // --- subtract for falling, add for rising edge
    if rising_edge {
      residual
    } else {
      residual * F::one().neg()
    }
  }
}

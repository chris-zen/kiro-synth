mod poly;
mod table;
mod table_blackman;
mod table_blackman_harris;
mod table_hamm;
mod table_hann;
mod table_rect;
mod table_tri;
mod table_welch;

pub use poly::PolyBLEP;
pub use table::BLEP;
pub use table_blackman::BLEP_8_BLACKMAN;
pub use table_blackman_harris::BLEP_8_BLACKMAN_HARRIS;
pub use table_hamm::BLEP_8_HAMM;
pub use table_hann::BLEP_8_HANN;
pub use table_rect::BLEP_8_RECT;
pub use table_tri::BLEP_8_TRI;
pub use table_welch::BLEP_8_WELCH;

use crate::float::Float;
use crate::funcs::interpolation::linear_interpolation;

pub trait TableBLEP {
  fn residual<F: Float>(
    &self,
    modulo: F,
    phase_inc: F,
    height: F,
    rising_edge: bool,
    points_per_side: usize,
    interpolate: bool,
  ) -> F;
}

impl TableBLEP for [f32; 4096] {
  fn residual<F: Float>(
    &self,
    modulo: F,
    phase_inc: F,
    height: F,
    rising_edge: bool,
    points_per_side: usize,
    interpolate: bool,
  ) -> F {
    // --- find the center of table (discontinuity location)
    let table_center = (F::from(self.len()).unwrap() / F::from(2.0).unwrap()) - F::one();

    let points_per_side = F::from(points_per_side).unwrap();

    let mut i = F::one();
    while i <= points_per_side {
      // LEFT side of edge
      // -1 < t < 0
      if modulo > F::one() - i * phase_inc {
        // --- calculate distance from the discontinuity
        let t = (modulo - F::one()) / (points_per_side * phase_inc);

        // --- get table offset
        let table_offset = (F::one() + t) * table_center;
        let index = table_offset.trunc().to_usize().unwrap();

        // --- truncation
        let y1 = F::from(self[index]).unwrap();
        let blep = if interpolate {
          let y2 = F::from(self[index + 1]).unwrap();
          linear_interpolation(F::zero(), F::one(), y1, y2, table_offset.fract())
        } else {
          y1
        };

        return height * blep_residual_with_sign(blep, rising_edge);
      }

      // RIGHT side of discontinuity
      // 0 <= t < 1
      if modulo < i * phase_inc {
        // --- calculate distance from the discontinuity
        let t = modulo / (points_per_side * phase_inc);

        // --- get table offset
        let table_offset = t * table_center + (table_center + F::one());
        let index = table_offset.trunc().to_usize().unwrap();

        // truncation
        let y1 = F::from(self[index]).unwrap();
        let blep = if interpolate {
          let y2 = if index + 1 >= self.len() {
            F::from(self[0]).unwrap()
          } else {
            F::from(self[index + 1]).unwrap()
          };

          linear_interpolation(F::zero(), F::one(), y1, y2, table_offset.fract())
        } else {
          y1
        };

        return height * blep_residual_with_sign(blep, rising_edge);
      }

      i = i + F::one();
    }

    // no residual
    F::zero()
  }
}

/// The BLEP residual with the sign depending on whether it is a rising or a falling edge.
/// The residual needs to be subtracted for falling edges or added for rising edges.
fn blep_residual_with_sign<F: Float>(value: F, rising_edge: bool) -> F {
  if !rising_edge {
    value.neg()
  } else {
    value
  }
}

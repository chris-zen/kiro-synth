use crate::float::Float;
use crate::funcs::signal_polarity::unipolar_to_bipolar;
use crate::oscillators::clamp_modulo;
use crate::waveforms::Waveform;

/// DPW Triangle Wave Oscillator
/// https://sci-hub.se/https://ieeexplore.ieee.org/document/1395943
///
#[derive(Debug, Clone)]
pub struct TriangleDpw2x<F: Float> {
  z1: F,
}

impl<F: Float> Default for TriangleDpw2x<F> {
  fn default() -> Self {
    TriangleDpw2x { z1: F::zero() }
  }
}

impl<F: Float> TriangleDpw2x<F> {
  pub fn new() -> Self {
    Self::default()
  }

  fn dpw_triangle(&mut self, modulo: F, sign: F) -> F {
    let bipolar_modulo = unipolar_to_bipolar(modulo);
    let decimation = (F::one() - bipolar_modulo * bipolar_modulo) * sign;
    let signal = decimation - self.z1;
    self.z1 = decimation;
    signal
  }
}

impl<F: Float> Waveform<F> for TriangleDpw2x<F> {
  fn reset(&mut self) {
    self.z1 = F::zero();
  }

  fn generate(&mut self, modulo: F, phase_inc: F) -> F {
    let modulo2x = modulo * F::val(2.0);

    let m1 = clamp_modulo(modulo2x);
    let sign1 = if modulo2x < F::one() {
      F::one()
    } else {
      F::one().neg()
    };
    let s1 = self.dpw_triangle(m1, sign1);

    let m2 = clamp_modulo(m1 + phase_inc * F::val(0.5));
    let sign2 = if m2 >= m1 { sign1 } else { sign1.neg() };
    let s2 = self.dpw_triangle(m2, sign2);

    let c = (F::val(8.0) * phase_inc * (F::one() - F::val(2.0) * phase_inc)).recip();

    (s1 + s2) * c
  }
}

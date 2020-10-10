use crate::float::Float;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub struct SineParabolic;

impl Default for SineParabolic {
  fn default() -> Self {
    SineParabolic
  }
}

impl SineParabolic {
  pub fn new() -> Self {
    Self::default()
  }
}

impl<F: Float> Waveform<F> for SineParabolic {
  fn generate(&mut self, modulo: F, _phase_inc: F) -> F {
    let angle = modulo * F::val(2.0) * F::PI - F::PI;
    angle.neg().parabolic_sine()
  }
}

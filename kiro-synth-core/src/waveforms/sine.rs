use crate::float::Float;
use crate::funcs::parabolic_sine::ParabolicSine;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub struct Sine;

impl<F: Float> Waveform<F> for Sine {
  fn generate(&mut self, modulo: F, phase_inc: F) -> F {
    let angle = modulo * F::val(2.0) * F::PI - F::PI;
    angle.neg().parabolic_sine()
  }
}

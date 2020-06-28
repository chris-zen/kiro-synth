use crate::float::Float;

pub mod lfo;
pub mod osc_freq_linear_mod;
pub mod osc_pitch_shift;
pub mod osc_waveform;
pub mod pitched_oscillator;

pub fn clamp_modulo<F: Float>(modulo: F) -> F {
  if modulo < F::zero() {
    modulo + F::one()
  } else if modulo >= F::one() {
    modulo - F::one()
  } else {
    modulo
  }
}

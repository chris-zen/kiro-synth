use crate::float::Float;

pub mod exponential;
pub mod saw_blep;
pub mod saw_trivial;
pub mod sine_parabolic;
pub mod square_trivial;
pub mod triangle_dpw2x;
pub mod triangle_trivial;

pub trait Waveform<F: Float> {
  fn initial_modulo(&self) -> F {
    F::zero()
  }

  fn reset(&mut self) {}

  fn generate(&mut self, modulo: F, phase_inc: F) -> F;
}

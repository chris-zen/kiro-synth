use crate::float::Float;

pub mod saw;
pub mod sine;
pub mod square;
pub mod triangle;

pub trait Waveform<F: Float> {
  fn generate(&self, modulo: F, phase_inc: F) -> F;
}

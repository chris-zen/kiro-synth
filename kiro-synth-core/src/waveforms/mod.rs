use num_traits::Float;

pub mod saw;
pub mod square;

pub trait Waveform<F: Float> {
  fn generate(&mut self, modulo: F, phase_inc: F) -> F;
}

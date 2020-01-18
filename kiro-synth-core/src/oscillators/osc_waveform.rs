use crate::waveforms::saw::Saw;
use crate::waveforms::Waveform;
use num_traits::Float;

#[derive(Debug, Clone)]
pub enum OscWaveform<F: Float> {
  Saw(Saw<F>),
}

impl<F> OscWaveform<F>
where
  F: Float,
{
  pub fn initial_modulo(&self) -> F {
    match self {
      OscWaveform::Saw(_) => F::from(0.5).unwrap(),
      //      _ => F::zero(),
    }
  }

  pub fn generate(&mut self, modulo: F, phase_inc: F) -> F {
    match self {
      OscWaveform::Saw(wf) => wf.generate(modulo, phase_inc),
    }
  }
}

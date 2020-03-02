use crate::float::Float;
use crate::waveforms::Waveform;
use crate::waveforms::saw::Saw;
use crate::waveforms::sine::Sine;

#[derive(Debug, Clone)]
pub enum OscWaveform<F: Float> {
  Sine(Sine),
  Saw(Saw<F>),
}

impl<F: Float> Default for OscWaveform<F> {
  fn default() -> Self {
    OscWaveform::Sine(Sine)
  }
}

impl<F> OscWaveform<F>
where
  F: Float,
{
  pub fn initial_modulo(&self) -> F {
    match self {
      OscWaveform::Sine(_) => F::zero(),
      OscWaveform::Saw(_) => F::from(0.5).unwrap(),
      //      _ => F::zero(),
    }
  }

  pub fn generate(&mut self, modulo: F, phase_inc: F) -> F {
    match self {
      OscWaveform::Sine(wf) => wf.generate(modulo, phase_inc),
      OscWaveform::Saw(wf) => wf.generate(modulo, phase_inc),
    }
  }
}

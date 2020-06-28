use crate::float::Float;
use crate::waveforms::saw_blep::SawBlep;
use crate::waveforms::saw_trivial::SawTrivial;
use crate::waveforms::sine_parabolic::SineParabolic;
use crate::waveforms::triangle_dpw2x::TriangleDpw2x;
use crate::waveforms::triangle_trivial::TriangleTrivial;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub enum OscWaveform<F: Float> {
  SineParabolic(SineParabolic),
  SawTrivial(SawTrivial),
  SawBlep(SawBlep<F>),
  TriangleTrivial(TriangleTrivial),
  TriangleDpw2x(TriangleDpw2x<F>),
}

impl<F: Float> Default for OscWaveform<F> {
  fn default() -> Self {
    OscWaveform::SineParabolic(SineParabolic)
  }
}

impl<F: Float> OscWaveform<F> {
  pub fn initial_modulo(&self) -> F {
    match self {
      OscWaveform::SineParabolic(wf) => wf.initial_modulo(),
      OscWaveform::SawTrivial(wf) => wf.initial_modulo(),
      OscWaveform::SawBlep(wf) => wf.initial_modulo(),
      OscWaveform::TriangleTrivial(wf) => wf.initial_modulo(),
      OscWaveform::TriangleDpw2x(wf) => wf.initial_modulo(),
    }
  }

  pub fn generate(&mut self, modulo: F, phase_inc: F) -> F {
    match self {
      OscWaveform::SineParabolic(wf) => wf.generate(modulo, phase_inc),
      OscWaveform::SawTrivial(wf) => wf.generate(modulo, phase_inc),
      OscWaveform::SawBlep(wf) => wf.generate(modulo, phase_inc),
      OscWaveform::TriangleTrivial(wf) => wf.generate(modulo, phase_inc),
      OscWaveform::TriangleDpw2x(wf) => wf.generate(modulo, phase_inc),
    }
  }
}

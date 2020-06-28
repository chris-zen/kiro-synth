use crate::float::Float;
use crate::oscillators::clamp_modulo;
use crate::oscillators::osc_waveform::OscWaveform;

// TODO add another waveform for quadrature phase output

// TODO Mode: free-running, synchronized, one-shot

#[derive(Debug, Clone)]
pub struct Lfo<F: Float> {
  waveform: OscWaveform<F>,
  rate: F,
  phase: F,
  depth: F,

  modulo: F,
  phase_inc: F,
  phase_inc_invalidated: bool,
  inv_sample_rate: F,
}

impl<F: Float> Lfo<F> {
  pub fn new(sample_rate: F) -> Self {
    let waveform = OscWaveform::default();
    let modulo = waveform.initial_modulo();
    Lfo {
      waveform,
      rate: F::one(),
      phase: F::zero(),
      depth: F::one(),

      modulo,
      phase_inc: F::zero(),
      phase_inc_invalidated: true,
      inv_sample_rate: sample_rate.recip(),
    }
  }

  /// Set the waveform
  pub fn set_waveform(&mut self, waveform: OscWaveform<F>) {
    self.waveform = waveform;
    self.reset_modulo();
    // FIXME figure out how to avoid clips after changing the waveform and the module
  }

  /// Set the rate
  pub fn set_rate(&mut self, rate: F) {
    self.rate = rate;
    self.phase_inc_invalidated = true;
  }

  /// Set the phase
  pub fn set_phase(&mut self, phase: F) {
    self.phase = phase;
  }

  /// Set the depth (amplitude)
  pub fn set_depth(&mut self, depth: F) {
    self.depth = depth;
  }

  /// Set the sample rate
  pub fn set_sample_rate(&mut self, sample_rate: F) {
    self.inv_sample_rate = sample_rate.recip();
    self.phase_inc_invalidated = true;
  }

  /// Reset the LFO
  pub fn reset(&mut self) {
    self.reset_modulo();
  }

  /// Generate the next value
  pub fn generate(&mut self) -> F {
    if self.phase_inc_invalidated {
      self.phase_inc = self.rate * self.inv_sample_rate;
    }

    let signal = self.waveform.generate(self.modulo, self.phase_inc);
    self.modulo = clamp_modulo(self.modulo + self.phase_inc);
    signal * self.depth
  }

  fn reset_modulo(&mut self) {
    self.modulo = clamp_modulo(self.waveform.initial_modulo() + self.phase);
  }
}

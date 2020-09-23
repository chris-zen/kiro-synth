use kiro_synth_core::float::Float;

use crate::waveforms::{LfoWaveforms, OscWaveforms};

#[derive(Debug, Clone, Default)]
pub struct SynthGlobals<F: Float> {
  pub osc_waveforms: OscWaveforms<F>,
  pub lfo_waveforms: LfoWaveforms<F>,
}

impl<F: Float> SynthGlobals<F> {
  pub fn new() -> Self {
    SynthGlobals {
      osc_waveforms: OscWaveforms::new(),
      lfo_waveforms: LfoWaveforms::new(),
    }
  }
}

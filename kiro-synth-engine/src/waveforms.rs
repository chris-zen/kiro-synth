use heapless::consts;
use heapless::Vec;

use kiro_synth_dsp::oscillators::osc_waveform::OscWaveform;
use kiro_synth_dsp::waveforms::saw_blep::{self, SawBlep};
use kiro_synth_dsp::waveforms::saw_trivial::SawTrivial;
use kiro_synth_dsp::waveforms::sine_parabolic::SineParabolic;
use kiro_synth_dsp::waveforms::triangle_dpw2x::TriangleDpw2x;
use kiro_synth_dsp::waveforms::triangle_trivial::TriangleTrivial;

use crate::float::Float;

type MaxWaveforms = consts::U8;

#[derive(Debug, Clone, Default)]
pub struct OscWaveforms<F: Float>(Vec<(&'static str, OscWaveform<F>), MaxWaveforms>);

impl<F: Float> OscWaveforms<F> {
  pub fn new() -> Self {
    let mut waveforms: Vec<(&'static str, OscWaveform<F>), MaxWaveforms> = heapless::Vec::new();

    waveforms
      .extend_from_slice(&[
        ("sin", OscWaveform::SineParabolic(SineParabolic)),
        ("tri", OscWaveform::TriangleDpw2x(TriangleDpw2x::default())),
        (
          "saw",
          OscWaveform::SawBlep(
            SawBlep::default()
              .with_mode(saw_blep::Mode::Bipolar)
              .with_correction(saw_blep::Correction::EightPointBlepWithInterpolation),
          ),
        ),
      ])
      .ok();

    OscWaveforms(waveforms)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn name(&self, index: usize) -> &'static str {
    self.0[index].0
  }

  pub fn waveform(&self, index: usize) -> &OscWaveform<F> {
    &self.0[index].1
  }
}

#[derive(Debug, Clone, Default)]
pub struct LfoWaveforms<F: Float>(Vec<(&'static str, OscWaveform<F>), MaxWaveforms>);

impl<F: Float> LfoWaveforms<F> {
  pub fn new() -> Self {
    let mut waveforms: Vec<(&'static str, OscWaveform<F>), MaxWaveforms> = heapless::Vec::new();
    waveforms
      .extend_from_slice(&[
        ("sin", OscWaveform::SineParabolic(SineParabolic)),
        (
          "tri",
          OscWaveform::TriangleTrivial(TriangleTrivial::default()),
        ),
        ("saw", OscWaveform::SawTrivial(SawTrivial::default())),
      ])
      .ok();
    LfoWaveforms(waveforms)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn name(&self, index: usize) -> &'static str {
    self.0[index].0
  }

  pub fn waveform(&self, index: usize) -> &OscWaveform<F> {
    &self.0[index].1
  }
}

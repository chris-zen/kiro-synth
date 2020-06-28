use heapless::consts;
use heapless::Vec;

use kiro_synth_core::oscillators::osc_waveform::OscWaveform;
use kiro_synth_core::waveforms::saw_blep::{self, SawBlep};
use kiro_synth_core::waveforms::saw_trivial::SawTrivial;
use kiro_synth_core::waveforms::sine_parabolic::SineParabolic;
use kiro_synth_core::waveforms::triangle_dpw2x::TriangleDpw2x;
use kiro_synth_core::waveforms::triangle_trivial::TriangleTrivial;

use crate::float::Float;

type MaxWaveforms = consts::U8;

#[derive(Debug, Clone)]
pub struct OscWaveforms<F: Float>(Vec<(&'static str, OscWaveform<F>), MaxWaveforms>);

impl<F: Float> OscWaveforms<F> {
  pub fn new() -> Self {
    let mut waveforms: Vec<(&'static str, OscWaveform<F>), MaxWaveforms> = heapless::Vec::new();
    drop(
      waveforms.extend_from_slice(&[
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
      ]),
    );
    OscWaveforms(waveforms)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn name(&self, index: usize) -> &'static str {
    self.0[index].0
  }

  pub fn waveform(&self, index: usize) -> &OscWaveform<F> {
    &self.0[index].1
  }
}

#[derive(Debug, Clone)]
pub struct LfoWaveforms<F: Float>(Vec<(&'static str, OscWaveform<F>), MaxWaveforms>);

impl<F: Float> LfoWaveforms<F> {
  pub fn new() -> Self {
    let mut waveforms: Vec<(&'static str, OscWaveform<F>), MaxWaveforms> = heapless::Vec::new();
    drop(waveforms.extend_from_slice(&[
      ("sin", OscWaveform::SineParabolic(SineParabolic)),
      (
        "tri",
        OscWaveform::TriangleTrivial(TriangleTrivial::default()),
      ),
      ("saw", OscWaveform::SawTrivial(SawTrivial::default())),
    ]));
    LfoWaveforms(waveforms)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn name(&self, index: usize) -> &'static str {
    self.0[index].0
  }

  pub fn waveform(&self, index: usize) -> &OscWaveform<F> {
    &self.0[index].1
  }
}

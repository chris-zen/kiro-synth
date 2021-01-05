use kiro_synth_dsp::oscillators::osc_waveform::OscWaveform;
use kiro_synth_dsp::oscillators::pitched_oscillator::PitchedOscillator;
use kiro_synth_dsp::waveforms::saw_blep::{self, SawBlep};
use kiro_synth_dsp::waveforms::sine_parabolic::SineParabolic;
use kiro_synth_dsp::waveforms::triangle_dpw2x::TriangleDpw2x;

use kiro_audio_engine::{Processor, ProcessorContext};
use kiro_audio_graph::{AudioDescriptor, NodeDescriptor, ParamDescriptor};

pub struct AnalogOsc {
  waveforms: [OscWaveform<f32>; 3],
  osc: PitchedOscillator<f32>,
}

impl AnalogOsc {
  pub const NUM_SHAPES: usize = 3;

  pub const SHAPE: &'static str = "shape";
  pub const OCTAVES: &'static str = "octaves";
  pub const SEMITONES: &'static str = "semitones";
  pub const CENTS: &'static str = "cents";
  pub const PITCH_BEND: &'static str = "pitch-bend";
  pub const AMPLITUDE: &'static str = "amplitude";

  pub fn node_descriptor() -> NodeDescriptor {
    NodeDescriptor::new("analog-osc")
      .static_audio_outputs(vec![AudioDescriptor::new("out", 1)])
      .static_parameters(vec![
        ParamDescriptor::new(Self::SHAPE)
          .min(0.0)
          .max(Self::NUM_SHAPES as f32)
          .center(0.0)
          .initial(0.0),
        ParamDescriptor::new(Self::OCTAVES)
          .min(-8.0)
          .max(8.0)
          .center(0.0)
          .initial(0.0),
        ParamDescriptor::new(Self::SEMITONES)
          .min(-12.0)
          .max(12.0)
          .center(0.0)
          .initial(0.0),
        ParamDescriptor::new(Self::CENTS)
          .min(-100.0)
          .max(100.0)
          .center(0.0)
          .initial(0.0),
        ParamDescriptor::new(Self::PITCH_BEND)
          .min(-1.0)
          .max(1.0)
          .center(0.0)
          .initial(0.0),
        ParamDescriptor::new(Self::AMPLITUDE)
          .min(-1.0)
          .max(1.0)
          .center(0.0)
          .initial(1.0),
      ])
  }

  pub fn new(sample_rate: f32, freq: f32) -> Self {
    let waveforms: [OscWaveform<f32>; Self::NUM_SHAPES] = [
      OscWaveform::SineParabolic(SineParabolic),
      OscWaveform::TriangleDpw2x(TriangleDpw2x::default()),
      OscWaveform::SawBlep(
        SawBlep::default()
          .with_mode(saw_blep::Mode::Bipolar)
          .with_correction(saw_blep::Correction::EightPointBlepWithInterpolation),
      ),
    ];
    let osc = PitchedOscillator::new(sample_rate, waveforms[0].clone(), freq);
    Self { waveforms, osc }
  }
}

impl Processor for AnalogOsc {
  fn render(&mut self, context: &mut ProcessorContext) {
    let mut amplitude = context.parameter(0).iter();

    let output = context.audio_output(0).channel_mut(0);
    for (index, sample) in output.as_mut_slice().iter_mut().enumerate() {
      let amp = amplitude.next();
      if amplitude.updated() {
        amp.iter().for_each(|value| self.osc.set_amplitude(*value));
      }
      *sample = self.osc.generate();
    }
  }
}

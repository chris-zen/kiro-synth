use kiro_synth_dsp::oscillators::osc_waveform::OscWaveform;
use kiro_synth_dsp::oscillators::pitched_oscillator::PitchedOscillator;
use kiro_synth_dsp::waveforms::saw_blep::{self, SawBlep};
use kiro_synth_dsp::waveforms::sine_parabolic::SineParabolic;
use kiro_synth_dsp::waveforms::triangle_dpw2x::TriangleDpw2x;

use kiro_audio_engine::renderer::RenderContext;
use kiro_audio_engine::Processor;
use kiro_audio_graph::{AudioDescriptor, NodeDescriptor, ParamDescriptor};

pub struct AnalogOsc {
  waveforms: [OscWaveform<f32>; 3],
  osc: PitchedOscillator<f32>,
}

impl AnalogOsc {
  pub const NUM_SHAPES: usize = 3;

  pub const OUT: &'static str = "out";

  pub const SHAPE: &'static str = "shape";
  pub const FREQUENCY: &'static str = "frequency";
  pub const OCTAVES: &'static str = "octaves";
  pub const SEMITONES: &'static str = "semitones";
  pub const CENTS: &'static str = "cents";
  pub const PITCH_BEND: &'static str = "pitch-bend";
  pub const AMPLITUDE: &'static str = "amplitude";

  pub fn descriptor() -> NodeDescriptor {
    NodeDescriptor::new("analog-osc")
      .static_audio_outputs(vec![AudioDescriptor::new(Self::OUT, 1)])
      .static_parameters(vec![
        ParamDescriptor::new(Self::SHAPE)
          .min(0.0)
          .max(Self::NUM_SHAPES as f32)
          .center(0.0)
          .initial(0.0),
        ParamDescriptor::new(Self::FREQUENCY)
          .min(0.0)
          .max(22000.0)
          .center(0.0)
          .initial(220.0),
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

  pub fn new(sample_rate: f32) -> Self {
    let waveforms: [OscWaveform<f32>; Self::NUM_SHAPES] = [
      OscWaveform::SineParabolic(SineParabolic),
      OscWaveform::TriangleDpw2x(TriangleDpw2x::default()),
      OscWaveform::SawBlep(
        SawBlep::default()
          .with_mode(saw_blep::Mode::Bipolar)
          .with_correction(saw_blep::Correction::EightPointBlepWithInterpolation),
      ),
    ];
    let osc = PitchedOscillator::new(sample_rate, waveforms[0].clone(), 80.0);
    Self { waveforms, osc }
  }
}

impl Processor for AnalogOsc {
  fn render(&mut self, context: &mut RenderContext) {
    let mut shape = context.parameter(Self::SHAPE).iter();
    let mut frequency = context.parameter(Self::FREQUENCY).iter();
    let mut octaves = context.parameter(Self::OCTAVES).iter();
    let mut semitones = context.parameter(Self::SEMITONES).iter();
    let mut cents = context.parameter(Self::CENTS).iter();
    let mut pitch_bend = context.parameter(Self::PITCH_BEND).iter();
    let mut amplitude = context.parameter(Self::AMPLITUDE).iter();

    let mut output = context.audio_output(Self::OUT).channel_mut(0);
    for sample in output.as_mut_slice().iter_mut() {
      // if let Some(value) = shape.next() {
      //   let index = value.round().max(0.0) as usize;
      //   let waveform = &self.waveforms[index];
      //   self.osc.set_waveform(waveform.clone())
      // }

      if let Some(value) = frequency.next_if_updated(|| self.osc.get_pitch_frequency()) {
        self.osc.set_pitch_frequency(value)
      }

      if let Some(value) = octaves.next_if_updated(|| self.osc.get_octaves()) {
        self.osc.set_octaves(value);
      }

      if let Some(value) = semitones.next_if_updated(|| self.osc.get_semitones()) {
        self.osc.set_semitones(value);
      }

      if let Some(value) = cents.next_if_updated(|| self.osc.get_cents()) {
        self.osc.set_cents(value);
      }

      if let Some(value) = pitch_bend.next_if_updated(|| self.osc.get_pitch_bend()) {
        self.osc.set_pitch_bend(value);
      }

      if let Some(value) = amplitude.next_if_updated(|| self.osc.get_amplitude()) {
        self.osc.set_amplitude(value);
      }

      *sample = self.osc.generate();
    }
  }
}

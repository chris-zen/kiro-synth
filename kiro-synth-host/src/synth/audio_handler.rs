use generic_array::GenericArray;
use ringbuf::Producer;

use kiro_synth_core::meters::PeakMeter;
use kiro_synth_engine::program::MaxParams;
use kiro_synth_engine::synth::Synth;

use crate::audio::AudioHandler;

#[derive(Debug, Clone)]
pub struct SynthAudioLevels {
  pub peak: f32,
  pub level: f32,
}

impl Default for SynthAudioLevels {
  fn default() -> Self {
    SynthAudioLevels {
      peak: 0.0,
      level: 0.0,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SynthFeedback {
  pub num_active_voices: usize,
  pub modulations: GenericArray<f32, MaxParams>,
  pub left_levels: SynthAudioLevels,
  pub right_levels: SynthAudioLevels,
}

pub struct SynthAudioHandler<'a> {
  synth: Synth<'a, f32>,
  feedback: Producer<SynthFeedback>,
  left_level: PeakMeter<f32>,
  right_level: PeakMeter<f32>,
}

impl<'a> SynthAudioHandler<'a> {
  pub fn new(synth: Synth<'a, f32>, feedback: Producer<SynthFeedback>) -> Self {
    let sample_rate = synth.get_sample_rate();
    SynthAudioHandler {
      synth,
      feedback,
      left_level: PeakMeter::new(sample_rate, 0.7, 24.0),
      right_level: PeakMeter::new(sample_rate, 0.7, 24.0),
    }
  }
}

impl<'a> AudioHandler for SynthAudioHandler<'a> {
  fn prepare(&mut self, _len: usize) {
    self.synth.prepare();
  }

  fn next(&mut self) -> (f32, f32) {
    let (left, right) = self.synth.process();
    self.left_level.process(left);
    self.right_level.process(right);
    (left, right)
  }

  fn finalize(&mut self) {
    let mut modulations = GenericArray::default();
    if let Some(voice) = self.synth.get_last_voice() {
      if !self.feedback.is_full() {
        let signals = voice.get_signals();
        let program = self.synth.get_program();
        for (index, param) in program.get_params().iter().enumerate() {
          let signal_index: usize = param.mod_signal_ref.into();
          modulations[index] = signals[signal_index].get();
        }
      }
    }
    let num_active_voices = self.synth.get_num_active_voices();
    let feedback = SynthFeedback {
      num_active_voices,
      modulations,
      left_levels: SynthAudioLevels {
        peak: self.left_level.get_peak(),
        level: self.left_level.get_level(),
      },
      right_levels: SynthAudioLevels {
        peak: self.right_level.get_peak(),
        level: self.right_level.get_level(),
      },
    };
    self.feedback.push(feedback).unwrap_or_default();
  }
}

use ringbuf::Producer;
use generic_array::GenericArray;

use kiro_synth_engine::program::MaxParams;
use kiro_synth_engine::synth::Synth;

use crate::audio::AudioHandler;


#[derive(Debug, Clone)]
pub struct SynthFeedback {
  pub num_active_voices: usize,
  pub modulations: GenericArray<f32, MaxParams>
}

pub struct SynthAudioHandler<'a> {
  synth: Synth<'a, f32>,
  feedback: Producer<SynthFeedback>,
}

impl<'a> SynthAudioHandler<'a> {
  pub fn new(synth: Synth<'a, f32>, feedback: Producer<SynthFeedback>) -> Self {
    SynthAudioHandler {
      synth,
      feedback,
    }
  }
}

impl<'a> AudioHandler for SynthAudioHandler<'a> {

  fn prepare(&mut self, _len: usize) {
    self.synth.prepare();
  }

  fn next(&mut self) -> (f32, f32) {
    self.synth.process()
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
    };
    self.feedback.push(feedback).unwrap_or_default();
  }
}

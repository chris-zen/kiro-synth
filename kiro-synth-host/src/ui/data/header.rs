use druid::{Data, Lens};

use crate::synth::{SynthAudioLevels, SynthClientMutex, SynthFeedback};

#[derive(Debug, Clone, Copy, PartialEq, Data)]
pub enum SelectedView {
  Presets,
  Synth,
  Effects,
}

impl SelectedView {
  pub fn title(&self) -> &'static str {
    match *self {
      SelectedView::Presets => "Presets",
      SelectedView::Synth => "Synth",
      SelectedView::Effects => "Effects",
    }
  }
}

#[derive(Debug, Clone, Data)]
pub struct AudioLevel {
  pub peak: f64,
  pub level: f64,
}

impl Default for AudioLevel {
  fn default() -> Self {
    AudioLevel {
      peak: f64::NEG_INFINITY,
      level: f64::NEG_INFINITY,
    }
  }
}

impl AudioLevel {
  pub fn new(level: &SynthAudioLevels) -> Self {
    AudioLevel {
      peak: level.peak as f64,
      level: level.level as f64,
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Header {
  pub selected_view: SelectedView,

  pub active_voices: usize,

  pub left_level: AudioLevel,
  pub right_level: AudioLevel,

  #[data(ignore)]
  pub synth_client: SynthClientMutex<f32>,
}

impl Header {
  pub fn new(synth_client: SynthClientMutex<f32>) -> Self {
    Header {
      selected_view: SelectedView::Synth,

      active_voices: 0,

      left_level: AudioLevel::default(),
      right_level: AudioLevel::default(),

      synth_client,
    }
  }

  pub fn update_feedback(&mut self, feedback: &SynthFeedback) {
    self.active_voices = feedback.num_active_voices;
    self.left_level = AudioLevel::new(&feedback.left_levels);
    self.right_level = AudioLevel::new(&feedback.right_levels);
  }
}

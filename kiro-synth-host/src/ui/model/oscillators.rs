use std::sync::{Arc, Mutex};

use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::Program;

use crate::program::params::OscParams;
use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, Param};

pub struct OscFromSynth;

impl Lens<SynthModel, Osc> for OscFromSynth {
  fn with<V, F: FnOnce(&Osc) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.osc[data.osc_index])
  }

  fn with_mut<V, F: FnOnce(&mut Osc) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.osc[data.osc_index])
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Osc {
  pub shape: Param,
  pub octaves: Param,
  pub semitones: Param,
  pub cents: Param,
  pub amplitude: Param,
}

impl Osc {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &OscParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Osc {
      shape: Param::new(program, &params.shape, synth_client.clone()),
      octaves: Param::new(program, &params.octaves, synth_client.clone()).with_origin(0.0),
      semitones: Param::new(program, &params.semitones, synth_client.clone()).with_origin(0.0),
      cents: Param::new(program, &params.cents, synth_client.clone()).with_origin(0.0),
      amplitude: Param::new(program, &params.amplitude, synth_client.clone()),
    }
  }
}

use std::sync::{Arc, Mutex, PoisonError, MutexGuard};

use druid::{Data, Lens};
use druid::im::{vector, Vector};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{ParamRef, Program, Param as ProgParam};

use crate::program::kiro::KiroModule;
use crate::program::params::{OscParams, EnvGenParams, FilterParams, DcaParams, LfoParams};
use crate::synth::SynthClient;
use crate::ui::widgets::knob::KnobData;
use crate::ui::model::{SynthModel, Param};

pub struct EgFromSynth;

impl Lens<SynthModel, EnvGen> for EgFromSynth {
  fn with<V, F: FnOnce(&EnvGen) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.eg[data.mod_index])
  }

  fn with_mut<V, F: FnOnce(&mut EnvGen) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.eg[data.mod_index])
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct EnvGen {
  pub attack: Param,
  pub decay: Param,
  pub sustain: Param,
  pub release: Param,
  pub mode: Param,
  pub legato: Param,
  pub reset_to_zero: Param,
  pub dca_intensity: Param,
}

impl EnvGen {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &EnvGenParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    EnvGen {
      attack: Param::new(program, &params.attack, synth_client.clone()),
      decay: Param::new(program, &params.decay, synth_client.clone()),
      sustain: Param::new(program, &params.sustain, synth_client.clone()),
      release: Param::new(program, &params.release, synth_client.clone()),
      mode: Param::new(program, &params.mode, synth_client.clone()),
      legato: Param::new(program, &params.legato, synth_client.clone()),
      reset_to_zero: Param::new(program, &params.reset_to_zero, synth_client.clone()),
      dca_intensity: Param::new(program, &params.dca_mod, synth_client.clone()),
    }
  }
}

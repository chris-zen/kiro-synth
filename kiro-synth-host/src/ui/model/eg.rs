use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::Program;

use crate::synth::program::params::EnvGenParams;
use crate::synth::SynthClientMutex;
use crate::ui::model::{Param, Synth};

pub struct EgFromSynth;

impl Lens<Synth, EnvGen> for EgFromSynth {
  fn with<V, F: FnOnce(&EnvGen) -> V>(&self, data: &Synth, f: F) -> V {
    f(&data.eg[data.mod_index])
  }

  fn with_mut<V, F: FnOnce(&mut EnvGen) -> V>(&self, data: &mut Synth, f: F) -> V {
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
  pub fn new<'a, F: Float + 'static>(
    program: &Program<'a, F>,
    params: &EnvGenParams,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
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

  pub fn for_each_modulated_param(&mut self, apply: &impl Fn(&mut Param)) {
    apply(&mut self.attack);
    apply(&mut self.decay);
    apply(&mut self.sustain);
    apply(&mut self.release);
    apply(&mut self.dca_intensity);
  }
}

use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::Program;

use crate::synth::program::params::LfoParams;
use crate::synth::SynthClientMutex;
use crate::ui::model::{Param, Synth};

pub struct LfoFromSynth;

impl Lens<Synth, Lfo> for LfoFromSynth {
  fn with<V, F: FnOnce(&Lfo) -> V>(&self, data: &Synth, f: F) -> V {
    f(&data.lfo[data.mod_index - data.eg.len()])
  }

  fn with_mut<V, F: FnOnce(&mut Lfo) -> V>(&self, data: &mut Synth, f: F) -> V {
    f(&mut data.lfo[data.mod_index - data.eg.len()])
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Lfo {
  pub shape: Param,
  pub rate: Param,
  pub phase: Param,
  pub depth: Param,
}

impl Lfo {
  pub fn new<'a, F: Float + 'static>(
    program: &Program<'a, F>,
    params: &LfoParams,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    Lfo {
      shape: Param::new(program, &params.shape, synth_client.clone()),
      rate: Param::new(program, &params.rate, synth_client.clone()),
      phase: Param::new(program, &params.phase, synth_client.clone()),
      depth: Param::new(program, &params.depth, synth_client),
    }
  }

  pub fn for_each_modulated_param(&mut self, apply: &impl Fn(&mut Param)) {
    apply(&mut self.rate);
    apply(&mut self.phase);
    apply(&mut self.depth);
  }
}

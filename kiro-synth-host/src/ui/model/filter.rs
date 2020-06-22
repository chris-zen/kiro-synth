use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::Program;

use crate::program::params::FilterParams;
use crate::synth::SynthClientMutex;
use crate::ui::model::{Synth, Param};


pub struct FilterFromSynth;

impl Lens<Synth, Filter> for FilterFromSynth {
  fn with<V, F: FnOnce(&Filter) -> V>(&self, data: &Synth, f: F) -> V {
    f(&data.filter[data.filter_index])
  }

  fn with_mut<V, F: FnOnce(&mut Filter) -> V>(&self, data: &mut Synth, f: F) -> V {
    f(&mut data.filter[data.filter_index])
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Filter {
  pub mode: Param,
  pub freq: Param,
  pub q: Param,
}

impl Filter {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &FilterParams,
                                     synth_client: SynthClientMutex<f32>) -> Self {
    Filter {
      mode: Param::new(program, &params.mode, synth_client.clone()),
      freq: Param::new(program, &params.freq, synth_client.clone()),
      q: Param::new(program, &params.q, synth_client.clone()),
    }
  }

  pub fn for_each_modulated_param(&mut self, apply: &impl Fn(&mut Param)) {
    apply(&mut self.freq);
    apply(&mut self.q);
  }
}

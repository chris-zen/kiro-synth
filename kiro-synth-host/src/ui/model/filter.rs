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


pub struct FilterFromSynth;

impl Lens<SynthModel, Filter> for FilterFromSynth {
  fn with<V, F: FnOnce(&Filter) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.filter[data.filter_index])
  }

  fn with_mut<V, F: FnOnce(&mut Filter) -> V>(&self, data: &mut SynthModel, f: F) -> V {
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
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Filter {
      mode: Param::new(program, &params.mode, synth_client.clone()),
      freq: Param::new(program, &params.freq, synth_client.clone()),
      q: Param::new(program, &params.q, synth_client.clone()),
    }
  }
}

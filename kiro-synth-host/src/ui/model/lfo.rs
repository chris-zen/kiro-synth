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


pub struct LfoFromSynth;

impl Lens<SynthModel, Lfo> for LfoFromSynth {
  fn with<V, F: FnOnce(&Lfo) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.lfo[data.mod_index - data.eg.len()])
  }

  fn with_mut<V, F: FnOnce(&mut Lfo) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.lfo[data.mod_index - data.eg.len()])
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Lfo {
  pub shape: Param,
  pub rate: Param,
  pub phase: Param,
  pub depth: Param,
  pub osc_pitch_mod: Param,
  pub filter_cutoff_mod: Param,
  pub dca_amp_mod: Param,
  pub dca_pan_mod: Param,
}

impl Lfo {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &LfoParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Lfo {
      shape: Param::new(program, &params.shape, synth_client.clone()),
      rate: Param::new(program, &params.rate, synth_client.clone()),
      phase: Param::new(program, &params.phase, synth_client.clone()),
      depth: Param::new(program, &params.depth, synth_client.clone()),
      osc_pitch_mod: Param::new(program, &params.modulation.osc_pitch, synth_client.clone()).with_origin(0.0),
      filter_cutoff_mod: Param::new(program, &params.modulation.filter_cutoff, synth_client.clone()).with_origin(0.0),
      dca_amp_mod: Param::new(program, &params.modulation.dca_amp, synth_client.clone()),
      dca_pan_mod: Param::new(program, &params.modulation.dca_pan, synth_client.clone()),
    }
  }
}

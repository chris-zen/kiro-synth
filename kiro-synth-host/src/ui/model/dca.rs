use std::sync::{Arc, Mutex, PoisonError, MutexGuard};

use druid::{Data, Lens};
use druid::im::{vector, Vector};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{ParamRef, Program, Param as ProgParam};

use crate::program::kiro::KiroModule;
use crate::program::params::{OscParams, EnvGenParams, FilterParams, DcaParams, LfoParams};
use crate::synth::SynthClient;
use crate::ui::widgets::knob::KnobData;
use crate::ui::model::Param;


#[derive(Debug, Clone, Data, Lens)]
pub struct Dca {
  pub amplitude: Param,
  pub pan: Param,
}

impl Dca {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &DcaParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Dca {
      amplitude: Param::new(program, &params.amplitude, synth_client.clone()),
      pan: Param::new(program, &params.pan, synth_client.clone()).with_origin(0.0),
    }
  }
}

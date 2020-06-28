use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::Program;

use crate::synth::program::params::DcaParams;
use crate::synth::SynthClientMutex;
use crate::ui::model::Param;

#[derive(Debug, Clone, Data, Lens)]
pub struct Dca {
  pub amplitude: Param,
  pub pan: Param,
}

impl Dca {
  pub fn new<'a, F: Float + 'static>(
    program: &Program<'a, F>,
    params: &DcaParams,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    Dca {
      amplitude: Param::new(program, &params.amplitude, synth_client.clone()),
      pan: Param::new(program, &params.pan, synth_client.clone()).with_origin(0.0),
    }
  }

  pub fn for_each_modulated_param(&mut self, apply: &impl Fn(&mut Param)) {
    apply(&mut self.amplitude);
    apply(&mut self.pan);
  }
}

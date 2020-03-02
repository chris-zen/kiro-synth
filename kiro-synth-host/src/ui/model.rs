use druid::{Data, Lens};

use crate::ui::knob::KnobModel;
use crate::ui::SynthParams;
use crate::ui::params::OscParamsInfo;

#[derive(Debug, Clone, Data, Lens)]
pub struct OscModel {
  amplitude: KnobModel,
  shape: KnobModel,
  octave: KnobModel,
  semitones: KnobModel,
  cents: KnobModel,
}

impl OscModel {
  pub fn new(params: &OscParamsInfo) -> Self {
    OscModel {
      amplitude: KnobModel::new(params.amplitude.initial_value, 0.0),
      shape: KnobModel::new(params.shape.initial_value, 0.0),
      octave: KnobModel::new(params.octave.initial_value, 0.0),
      semitones: KnobModel::new(params.semitones.initial_value, 0.0),
      cents: KnobModel::new(params.cents.initial_value, 0.0),
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Model {
  osc1: OscModel,
  osc2: OscModel,
}

impl Model {
  pub fn new(synth_params: &SynthParams) -> Self {
    Model {
      osc1: OscModel::new(&synth_params.osc1),
      osc2: OscModel::new(&synth_params.osc2),
    }
  }
}

use druid::{Data, Lens};

use crate::ui::knob::KnobModel;
use crate::ui::SynthParams;
use crate::ui::params::{OscParamsInfo, EgParamsInfo};

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
pub struct EgModel {
  attack: KnobModel,
  decay: KnobModel,
  sustain: KnobModel,
  release: KnobModel,
  mode: KnobModel,
  legato: KnobModel,
  reset_to_zero: KnobModel,
  dca_intensity: KnobModel,
}

impl EgModel {
  pub fn new(params: &EgParamsInfo) -> Self {
    EgModel {
      attack: KnobModel::new(params.attack.initial_value, 0.0),
      decay: KnobModel::new(params.decay.initial_value, 0.0),
      sustain: KnobModel::new(params.sustain.initial_value, 0.0),
      release: KnobModel::new(params.release.initial_value, 0.0),
      mode: KnobModel::new(params.mode.initial_value, 0.0),
      legato: KnobModel::new(params.legato.initial_value, 0.0),
      reset_to_zero: KnobModel::new(params.reset_to_zero.initial_value, 0.0),
      dca_intensity: KnobModel::new(params.dca_intensity.initial_value, 0.0),
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Model {
  osc1: OscModel,
  osc2: OscModel,
  eg1: EgModel,
}

impl Model {
  pub fn new(synth_params: &SynthParams) -> Self {
    Model {
      osc1: OscModel::new(&synth_params.osc1),
      osc2: OscModel::new(&synth_params.osc2),
      eg1: EgModel::new(&synth_params.eg1),
    }
  }
}

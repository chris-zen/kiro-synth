use kiro_synth_engine::program::{ParamRef, Param, Program};
use kiro_synth_core::float::Float;

use crate::program::params::{EnvGenParams, OscParams, FilterParams, DcaParams};
use crate::programs::KiroParams;

pub struct ParamInfo {
  pub param_ref: ParamRef,
  pub initial_value: f64,
  pub center: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
}

impl ParamInfo {
  pub fn new<F: Float>(param_info: Option<(ParamRef, &Param<F>)>) -> Self {
    let (param_ref, param) = param_info.unwrap();
    ParamInfo {
      param_ref,
      initial_value: param.values.initial_value.to_f64().unwrap(),
      center: param.values.min.to_f64().unwrap(),
      min: param.values.min.to_f64().unwrap(),
      max: param.values.max.to_f64().unwrap(),
      step: param.values.resolution.to_f64().unwrap(),
    }
  }
}

pub struct OscParamsInfo {
  pub amplitude: ParamInfo,
  pub shape: ParamInfo,
  pub octave: ParamInfo,
  pub semitones: ParamInfo,
  pub cents: ParamInfo,
}

pub struct SynthParams {
  pub osc1: OscParamsInfo,
  pub osc2: OscParamsInfo,
}

impl SynthParams {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, params: &KiroParams) -> Self {
    SynthParams {
      osc1: Self::osc_params(program, &params.osc3),
      osc2: Self::osc_params(program, &params.osc4),
    }
  }

  fn osc_params<'a, F: Float + 'static>(program: &Program<'a, F>, params: &OscParams) -> OscParamsInfo {
    OscParamsInfo {
      amplitude: ParamInfo::new(program.get_param(params.amplitude.reference)),
      shape: ParamInfo::new(program.get_param(params.shape.reference)),
      octave: ParamInfo::new(program.get_param(params.octave.reference)),
      semitones: ParamInfo::new(program.get_param(params.semitones.reference)),
      cents: ParamInfo::new(program.get_param(params.cents.reference)),
    }
  }
}

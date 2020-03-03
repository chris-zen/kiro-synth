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

pub struct EgParamsInfo {
  pub attack: ParamInfo,
  pub decay: ParamInfo,
  pub sustain: ParamInfo,
  pub release: ParamInfo,
  pub mode: ParamInfo,
  pub legato: ParamInfo,
  pub reset_to_zero: ParamInfo,
  pub dca_intensity: ParamInfo,
}

pub struct SynthParams {
  pub osc1: OscParamsInfo,
  pub osc2: OscParamsInfo,
  pub eg1: EgParamsInfo,
}

impl SynthParams {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, params: &KiroParams) -> Self {
    SynthParams {
      osc1: Self::osc_params(program, &params.osc3),
      osc2: Self::osc_params(program, &params.osc4),
      eg1: Self::eg_params(program, &params.eg1),
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

  fn eg_params<'a, F: Float + 'static>(program: &Program<'a, F>, params: &EnvGenParams) -> EgParamsInfo {
    EgParamsInfo {
      attack: ParamInfo::new(program.get_param(params.attack.reference)),
      decay: ParamInfo::new(program.get_param(params.decay.reference)),
      sustain: ParamInfo::new(program.get_param(params.sustain.reference)),
      release: ParamInfo::new(program.get_param(params.release.reference)),
      mode: ParamInfo::new(program.get_param(params.mode.reference)),
      legato: ParamInfo::new(program.get_param(params.legato.reference)),
      reset_to_zero: ParamInfo::new(program.get_param(params.reset_to_zero.reference)),
      dca_intensity: ParamInfo::new(program.get_param(params.dca_intensity.reference)),
    }
  }
}

use druid::{Data, Lens};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{ParamRef, Program};

use crate::ui::widgets::knob::KnobData;
use crate::program::kiro::KiroModule;
use crate::program::params::{OscParams, EnvGenParams, FilterParams, DcaParams};

pub struct OscFromSynth;

impl Lens<SynthModel, Osc> for OscFromSynth {
  fn with<V, F: FnOnce(&Osc) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.osc[data.osc_index])
  }

  fn with_mut<V, F: FnOnce(&mut Osc) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.osc[data.osc_index])
  }
}

pub struct EgFromSynth;

impl Lens<SynthModel, EnvGen> for EgFromSynth {
  fn with<V, F: FnOnce(&EnvGen) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.eg[data.eg_index])
  }

  fn with_mut<V, F: FnOnce(&mut EnvGen) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.eg[data.eg_index])
  }
}

pub struct FilterFromSynth;

impl Lens<SynthModel, Filter> for FilterFromSynth {
  fn with<V, F: FnOnce(&Filter) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.filter[data.filter_index])
  }

  fn with_mut<V, F: FnOnce(&mut Filter) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.filter[data.filter_index])
  }
}

pub struct ZeroIndex;

impl Lens<SynthModel, usize> for ZeroIndex {
  fn with<V, F: FnOnce(&usize) -> V>(&self, _data: &SynthModel, f: F) -> V {
    f(&0usize)
  }

  fn with_mut<V, F: FnOnce(&mut usize) -> V>(&self, _data: &mut SynthModel, f: F) -> V {
    f(&mut 0usize)
  }
}

pub struct KnobDataFromParam;

impl Lens<Param, KnobData> for KnobDataFromParam {
  fn with<V, F: FnOnce(&KnobData) -> V>(&self, data: &Param, f: F) -> V {
    f(&KnobData::new(data.value, data.modulation))
  }

  fn with_mut<V, F: FnOnce(&mut KnobData) -> V>(&self, data: &mut Param, f: F) -> V {
    let mut knob_data = KnobData::new(data.value, data.modulation);
    let result = f(&mut knob_data);
    data.value = knob_data.value;
    // we don't need to copy back the modulation as it is a read-only attribute for the Knob
    result
  }
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct Param {
  #[druid(same_fn = "PartialEq::eq")]
  pub param_ref: ParamRef,
  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub value: f64,
  pub modulation: f64,
}

impl Param {
  pub fn new<F: Float, P: Into<ParamRef>>(program: &Program<F>, param_ref: P) -> Self {
    let (param_ref, param) = program.get_param(param_ref.into()).unwrap();
    Param {
      param_ref,
      origin: param.values.min.to_f64().unwrap(), // TODO Add an equivalent to origin for ParamValues
      min: param.values.min.to_f64().unwrap(),
      max: param.values.max.to_f64().unwrap(),
      step: param.values.resolution.to_f64().unwrap(),
      value: param.values.initial_value.to_f64().unwrap(),
      modulation: 0.0,
    }
  }

  pub fn with_origin(self, origin: f64) -> Self {
    Param {
      origin,
      .. self
    }
  }
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct Osc {
  pub amplitude: Param,
  pub shape: Param,
  pub octaves: Param,
  pub semitones: Param,
  pub cents: Param,
}

impl Osc {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, params: &OscParams) -> Self {
    Osc {
      amplitude: Param::new(program, &params.amplitude),
      shape: Param::new(program, &params.shape),
      octaves: Param::new(program, &params.octaves).with_origin(0.0),
      semitones: Param::new(program, &params.semitones).with_origin(0.0),
      cents: Param::new(program, &params.cents).with_origin(0.0),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct EnvGen {
  pub attack: Param,
  pub decay: Param,
  pub sustain: Param,
  pub release: Param,
  pub mode: Param,
  pub legato: Param,
  pub reset_to_zero: Param,
  pub dca_intensity: Param,
}

impl EnvGen {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, params: &EnvGenParams) -> Self {
    EnvGen {
      attack: Param::new(program, &params.attack),
      decay: Param::new(program, &params.decay),
      sustain: Param::new(program, &params.sustain),
      release: Param::new(program, &params.release),
      mode: Param::new(program, &params.mode),
      legato: Param::new(program, &params.legato),
      reset_to_zero: Param::new(program, &params.reset_to_zero),
      dca_intensity: Param::new(program, &params.dca_intensity),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct Filter {
  pub mode: Param,
  pub freq: Param,
  pub q: Param,
}

impl Filter {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, params: &FilterParams) -> Self {
    Filter {
      mode: Param::new(program, &params.mode),
      freq: Param::new(program, &params.freq),
      q: Param::new(program, &params.q),
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Dca {
  pub amplitude: Param,
  pub pan: Param,
}

impl Dca {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, params: &DcaParams) -> Self {
    Dca {
      amplitude: Param::new(program, &params.amplitude),
      pan: Param::new(program, &params.pan).with_origin(0.0),
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct SynthModel {
  #[druid(same_fn = "PartialEq::eq")]
  pub osc: Vec<Osc>,
  pub osc_index: usize,

  #[druid(same_fn = "PartialEq::eq")]
  pub eg: Vec<EnvGen>,
  pub eg_index: usize,

  #[druid(same_fn = "PartialEq::eq")]
  pub filter: Vec<Filter>,
  pub filter_index: usize,

  pub dca: Dca,
}

impl SynthModel {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>, module: &KiroModule) -> Self {
    let params = &module.params;

    SynthModel {
      osc: vec![
        Osc::new(program, &params.osc3),
        Osc::new(program, &params.osc4),
      ],
      osc_index: 0,

      eg: vec![
        EnvGen::new(program, &params.eg1),
      ],
      eg_index: 0,

      filter: vec![
        Filter::new(program, &params.filt1),
      ],
      filter_index: 0,

      dca: Dca::new(program, &params.dca),
    }
  }
}

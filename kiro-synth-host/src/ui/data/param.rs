use druid::{Data, Lens};

use kiro_synth_dsp::float::Float;
use kiro_synth_engine::program::{Param as ProgParam, ParamRef, Program, SourceRef};

use crate::synth::SynthClientMutex;
use crate::ui::widgets::knob::{KnobData, KnobModulationData};
use std::sync::Arc;

pub struct KnobDataFromParam;

impl KnobDataFromParam {
  fn create_knob_data_from_param(data: &Param) -> KnobData<Param> {
    let modulation = &data.modulation;
    let config_source = modulation.config_source.map(|source_ref| source_ref.into());
    let knob_modulation = KnobModulationData::new(
      modulation.value,
      config_source,
      modulation.config_amount,
      modulation.total_amount,
    );
    KnobData::new(
      data.origin,
      data.min,
      data.max,
      data.step,
      data.value,
      data.clone(),
    )
    .with_modulation(knob_modulation)
  }
}

impl Lens<Param, KnobData<Param>> for KnobDataFromParam {
  fn with<V, F: FnOnce(&KnobData<Param>) -> V>(&self, data: &Param, f: F) -> V {
    let knob_data = Self::create_knob_data_from_param(data);
    f(&knob_data)
  }

  fn with_mut<V, F: FnOnce(&mut KnobData<Param>) -> V>(&self, data: &mut Param, f: F) -> V {
    let mut knob_data = Self::create_knob_data_from_param(data);
    let result = f(&mut knob_data);
    data.value = knob_data.value;
    data.modulation.config_amount = knob_data.modulation.config_amount;
    // we don't need to copy back the rest of attributes as they are read-only for the Knob
    result
  }
}

#[derive(Debug, Clone, Data)]
pub struct ParamModulation {
  /// The value of the modulation applied to the parameter coming from the audio thread in real time
  pub value: f64,

  /// When the knob is in config mode, it contains some source ref
  #[data(same_fn = "PartialEq::eq")]
  pub config_source: Option<SourceRef>,

  /// Amount of modulation from the selected source while in configuration mode
  pub config_amount: f64,

  /// Total amount of modulation applied to the parameter from all the connected sources
  pub total_amount: f64,
}

impl Default for ParamModulation {
  fn default() -> Self {
    ParamModulation {
      value: 0.0,
      config_source: None,
      config_amount: 0.0,
      total_amount: 0.0,
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Param {
  #[data(same_fn = "PartialEq::eq")]
  pub param_ref: ParamRef,

  pub name: Arc<String>,

  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub value: f64,

  pub modulation: ParamModulation,

  #[data(ignore)]
  pub synth_client: SynthClientMutex<f32>,
}

impl Param {
  pub fn new<F: Float, P: Into<ParamRef>>(
    program: &Program<F>,
    param_ref: P,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    let (param_ref, param) = program.get_param(param_ref.into()).unwrap();
    Self::from(param_ref, param, synth_client)
  }

  pub fn from<F: Float>(
    param_ref: ParamRef,
    param: &ProgParam<F>,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    Param {
      param_ref,
      name: Arc::new(param.id.to_string()),
      origin: param.values.origin.to_f64().unwrap(),
      min: param.values.min.to_f64().unwrap(),
      max: param.values.max.to_f64().unwrap(),
      step: param.values.resolution.to_f64().unwrap(),
      value: param.values.initial_value.to_f64().unwrap(),
      modulation: ParamModulation::default(),
      synth_client,
    }
  }

  pub fn with_origin(mut self, origin: f64) -> Self {
    self.origin = origin;
    self
  }
}

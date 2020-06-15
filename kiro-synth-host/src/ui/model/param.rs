use std::sync::{Arc, Mutex, PoisonError, MutexGuard};

use druid::{Data, Lens};
use druid::im::{vector, Vector};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{ParamRef, Program, Param as ProgParam};

use crate::ui::widgets::knob::KnobData;
use crate::program::kiro::KiroModule;
use crate::program::params::{OscParams, EnvGenParams, FilterParams, DcaParams, LfoParams};
use crate::synth::SynthClient;
use crate::ui::model::modulations::Modulations;


pub struct KnobDataFromParam;

impl Lens<Param, KnobData<Param>> for KnobDataFromParam {
  fn with<V, F: FnOnce(&KnobData<Param>) -> V>(&self, data: &Param, f: F) -> V {
    let knob_data = KnobData::new(data.origin, data.min, data.max, data.step, data.value, data.clone())
        .with_modulation_value(data.modulation);
    f(&knob_data)
  }

  fn with_mut<V, F: FnOnce(&mut KnobData<Param>) -> V>(&self, data: &mut Param, f: F) -> V {
    let mut knob_data = KnobData::new(data.origin, data.min, data.max, data.step, data.value, data.clone())
        .with_modulation_value(data.modulation);
    let result = f(&mut knob_data);
    data.value = knob_data.value;
    // we don't need to copy back the rest of attributes as they are read-only for the Knob
    result
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Param {
  #[data(same_fn = "PartialEq::eq")]
  pub param_ref: ParamRef,
  pub origin: f64,
  pub min: f64,
  pub max: f64,
  pub step: f64,
  pub value: f64,
  pub modulation: f64,
  #[data(ignore)]
  pub synth_client: Arc<Mutex<SynthClient<f32>>>,
}

impl Param {
  pub fn new<F: Float, P: Into<ParamRef>>(program: &Program<F>,
                                          param_ref: P,
                                          synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {

    let (param_ref, param) = program.get_param(param_ref.into()).unwrap();
    Self::from(param_ref, param, synth_client)
  }

  pub fn from<F: Float>(param_ref: ParamRef,
                        param: &ProgParam<F>,
                        synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Param {
      param_ref,
      origin: param.values.origin.to_f64().unwrap(),
      min: param.values.min.to_f64().unwrap(),
      max: param.values.max.to_f64().unwrap(),
      step: param.values.resolution.to_f64().unwrap(),
      value: param.values.initial_value.to_f64().unwrap(),
      modulation: 0.0,
      synth_client,
    }
  }

  pub fn with_origin(self, origin: f64) -> Self {
    Param {
      origin,
      .. self
    }
  }

  pub fn send_value(&self, value: f64) -> Result<(), PoisonError<MutexGuard<'_, SynthClient<f32>>>> {
    self.synth_client.lock()
        .map(|mut client| client.send_param_value(self.param_ref, value as f32))
  }
}

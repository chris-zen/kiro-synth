pub mod modulations;

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
    f(&data.eg[data.mod_index])
  }

  fn with_mut<V, F: FnOnce(&mut EnvGen) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.eg[data.mod_index])
  }
}

pub struct LfoFromSynth;

impl Lens<SynthModel, Lfo> for LfoFromSynth {
  fn with<V, F: FnOnce(&Lfo) -> V>(&self, data: &SynthModel, f: F) -> V {
    f(&data.lfo[data.mod_index - data.eg.len()])
  }

  fn with_mut<V, F: FnOnce(&mut Lfo) -> V>(&self, data: &mut SynthModel, f: F) -> V {
    f(&mut data.lfo[data.mod_index - data.eg.len()])
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
    f(&KnobData::new(data.origin, data.min, data.max, data.step, data.value, data.modulation))
  }

  fn with_mut<V, F: FnOnce(&mut KnobData) -> V>(&self, data: &mut Param, f: F) -> V {
    let mut knob_data = KnobData::new(data.origin, data.min, data.max, data.step, data.value, data.modulation);
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

#[derive(Debug, Clone, Data, Lens)]
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
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &EnvGenParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    EnvGen {
      attack: Param::new(program, &params.attack, synth_client.clone()),
      decay: Param::new(program, &params.decay, synth_client.clone()),
      sustain: Param::new(program, &params.sustain, synth_client.clone()),
      release: Param::new(program, &params.release, synth_client.clone()),
      mode: Param::new(program, &params.mode, synth_client.clone()),
      legato: Param::new(program, &params.legato, synth_client.clone()),
      reset_to_zero: Param::new(program, &params.reset_to_zero, synth_client.clone()),
      dca_intensity: Param::new(program, &params.dca_mod, synth_client.clone()),
    }
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

#[derive(Debug, Clone, Data, Lens)]
pub struct Osc {
  pub shape: Param,
  pub octaves: Param,
  pub semitones: Param,
  pub cents: Param,
  pub amplitude: Param,
}

impl Osc {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &OscParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Osc {
      shape: Param::new(program, &params.shape, synth_client.clone()),
      octaves: Param::new(program, &params.octaves, synth_client.clone()).with_origin(0.0),
      semitones: Param::new(program, &params.semitones, synth_client.clone()).with_origin(0.0),
      cents: Param::new(program, &params.cents, synth_client.clone()).with_origin(0.0),
      amplitude: Param::new(program, &params.amplitude, synth_client.clone()),
    }
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Filter {
  pub mode: Param,
  pub freq: Param,
  pub q: Param,
}

impl Filter {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     params: &FilterParams,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    Filter {
      mode: Param::new(program, &params.mode, synth_client.clone()),
      freq: Param::new(program, &params.freq, synth_client.clone()),
      q: Param::new(program, &params.q, synth_client.clone()),
    }
  }
}

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

#[derive(Debug, Clone, Data, Lens)]
pub struct SynthModel {

  pub osc: Vector<Osc>,
  pub osc_index: usize,

  pub mod_index: usize,

  pub eg: Vector<EnvGen>,

  pub lfo: Vector<Lfo>,

  pub filter: Vector<Filter>,
  pub filter_index: usize,

  pub dca: Dca,

  pub modulations: Modulations,
}

impl SynthModel {
  pub fn new<'a, F: Float + 'static>(program: &Program<'a, F>,
                                     module: &KiroModule,
                                     synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {

    let params = &module.params;

    SynthModel {
      osc: vector![
        Osc::new(program, &params.osc1, synth_client.clone()),
        Osc::new(program, &params.osc2, synth_client.clone()),
        Osc::new(program, &params.osc3, synth_client.clone()),
        Osc::new(program, &params.osc4, synth_client.clone()),
      ],
      osc_index: 0,

      mod_index: 0,

      eg: vector![
        EnvGen::new(program, &params.eg1, synth_client.clone()),
      ],

      lfo: vector![
        Lfo::new(program, &params.lfo1, synth_client.clone()),
        Lfo::new(program, &params.lfo2, synth_client.clone()),
      ],

      filter: vector![
        Filter::new(program, &params.filter1, synth_client.clone()),
      ],
      filter_index: 0,

      dca: Dca::new(program, &params.dca, synth_client.clone()),

      modulations: Modulations::new(program, module, synth_client.clone()),
    }
  }
}

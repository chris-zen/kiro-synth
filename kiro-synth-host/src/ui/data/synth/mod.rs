mod dca;
mod eg;
mod filter;
mod lfo;
mod oscillators;

use druid::im::{vector, Vector};
use druid::{Data, Lens};

use kiro_synth_dsp::float::Float;
use kiro_synth_engine::program::Program;

use crate::synth::program::kiro::KiroModule;
use crate::synth::{SynthClientMutex, SynthFeedback};
use crate::ui::data::param::Param;

pub use dca::Dca;
pub use eg::{EgFromSynth, EnvGen};
pub use filter::{Filter, FilterFromSynth};
pub use lfo::{Lfo, LfoFromSynth};
pub use oscillators::{Osc, OscFromSynth};

pub struct ZeroIndex;

impl Lens<Synth, usize> for ZeroIndex {
  fn with<V, F: FnOnce(&usize) -> V>(&self, _data: &Synth, f: F) -> V {
    f(&0usize)
  }

  fn with_mut<V, F: FnOnce(&mut usize) -> V>(&self, _data: &mut Synth, f: F) -> V {
    f(&mut 0usize)
  }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Synth {
  pub osc: Vector<Osc>,
  pub osc_index: usize,

  pub mod_index: usize,

  pub eg: Vector<EnvGen>,

  pub lfo: Vector<Lfo>,

  pub filter: Vector<Filter>,
  pub filter_index: usize,

  pub dca: Dca,

  #[data(ignore)]
  pub synth_client: SynthClientMutex<f32>,
}

impl Synth {
  pub fn new<'a, F: Float + 'static>(
    program: &Program<'a, F>,
    module: &KiroModule,
    synth_client: SynthClientMutex<f32>,
  ) -> Self {
    let params = &module.params;

    Synth {
      osc: vector![
        Osc::new(program, &params.osc1, synth_client.clone()),
        Osc::new(program, &params.osc2, synth_client.clone()),
        Osc::new(program, &params.osc3, synth_client.clone()),
        Osc::new(program, &params.osc4, synth_client.clone()),
      ],
      osc_index: 0,

      mod_index: 0,

      eg: vector![EnvGen::new(program, &params.eg1, synth_client.clone()),],

      lfo: vector![
        Lfo::new(program, &params.lfo1, synth_client.clone()),
        Lfo::new(program, &params.lfo2, synth_client.clone()),
      ],

      filter: vector![Filter::new(program, &params.filter1, synth_client.clone()),],
      filter_index: 0,

      dca: Dca::new(program, &params.dca, synth_client.clone()),

      synth_client,
    }
  }
}

impl<'a> Synth {
  pub fn update_feedback(&mut self, feedback: &SynthFeedback) {
    self.for_each_modulated_param(|param| {
      let param_index: usize = param.param_ref.into();
      let modulation = feedback.modulations[param_index];
      param.modulation.value = modulation as f64;
    });
  }

  pub fn for_each_modulated_param(&mut self, apply: impl Fn(&mut Param)) {
    for osc in self.osc.iter_mut() {
      osc.for_each_modulated_param(&apply);
    }
    for eg in self.eg.iter_mut() {
      eg.for_each_modulated_param(&apply);
    }
    for lfo in self.lfo.iter_mut() {
      lfo.for_each_modulated_param(&apply);
    }
    for filter in self.filter.iter_mut() {
      filter.for_each_modulated_param(&apply);
    }
    self.dca.for_each_modulated_param(&apply);
  }
}

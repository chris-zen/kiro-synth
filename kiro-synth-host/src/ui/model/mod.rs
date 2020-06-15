mod param;
mod eg;
mod lfo;
mod oscillators;
mod filter;
mod dca;
pub mod modulations;

use std::sync::{Arc, Mutex};

use druid::{Data, Lens};
use druid::im::{vector, Vector};

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, SourceRef};

use crate::program::kiro::KiroModule;
use crate::synth::SynthClient;

pub use param::{KnobDataFromParam, Param};
pub use eg::{EgFromSynth, EnvGen};
pub use lfo::{LfoFromSynth, Lfo};
pub use oscillators::{OscFromSynth, Osc};
pub use filter::{FilterFromSynth, Filter};
pub use dca::Dca;
pub use modulations::Modulations;


pub struct ZeroIndex;

impl Lens<SynthModel, usize> for ZeroIndex {
  fn with<V, F: FnOnce(&usize) -> V>(&self, _data: &SynthModel, f: F) -> V {
    f(&0usize)
  }

  fn with_mut<V, F: FnOnce(&mut usize) -> V>(&self, _data: &mut SynthModel, f: F) -> V {
    f(&mut 0usize)
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

impl SynthModel {
  pub fn start_modulations_config(&mut self, source_ref: SourceRef) {
    self.modulations.start_config(source_ref);

  }

  pub fn stop_modulations_config(&mut self, source_ref: SourceRef) {
    self.modulations.stop_config(source_ref);
  }
}
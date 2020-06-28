mod dca;
mod eg;
mod filter;
mod lfo;
pub mod modulations;
mod oscillators;
mod param;
mod synth;

use druid::Lens;

pub use dca::Dca;
pub use eg::{EgFromSynth, EnvGen};
pub use filter::{Filter, FilterFromSynth};
pub use lfo::{Lfo, LfoFromSynth};
pub use modulations::Modulations;
pub use oscillators::{Osc, OscFromSynth};
pub use param::{KnobDataFromParam, Param};
pub use synth::Synth;

pub struct ZeroIndex;

impl Lens<Synth, usize> for ZeroIndex {
  fn with<V, F: FnOnce(&usize) -> V>(&self, _data: &Synth, f: F) -> V {
    f(&0usize)
  }

  fn with_mut<V, F: FnOnce(&mut usize) -> V>(&self, _data: &mut Synth, f: F) -> V {
    f(&mut 0usize)
  }
}

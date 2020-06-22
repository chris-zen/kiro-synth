mod param;
mod eg;
mod lfo;
mod oscillators;
mod filter;
mod dca;
pub mod modulations;
mod synth;

use druid::Lens;

pub use param::{KnobDataFromParam, Param};
pub use eg::{EgFromSynth, EnvGen};
pub use lfo::{LfoFromSynth, Lfo};
pub use oscillators::{OscFromSynth, Osc};
pub use filter::{FilterFromSynth, Filter};
pub use dca::Dca;
pub use modulations::Modulations;
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

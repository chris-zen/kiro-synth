use kiro_synth_engine::program::{ParamBlock, ProgramBuilder, Block};
use kiro_synth_core::float::Float;

macro_rules! param_blocks {
  ( $name:ident, $( $param_block:ident ),* $(, $( [$nested:ident] ),* )? ) => {
    impl $name {
      pub fn add_param_blocks<F: Float>(&self, program: &mut ProgramBuilder<F>) {
        $(
          program.block(Block::Param(self.$param_block.clone()));
        )*

        $(
          $(
            self.$nested.add_param_blocks(program)
          )*
        )?
      }
    }
  };
}

pub struct EnvGenParams {
  pub attack: ParamBlock,
  pub decay: ParamBlock,
  pub sustain: ParamBlock,
  pub release: ParamBlock,
  pub mode: ParamBlock,
  pub legato: ParamBlock,
  pub reset_to_zero: ParamBlock,
  pub dca_mod: ParamBlock,
}

param_blocks!(EnvGenParams, attack, decay, sustain, release, mode, legato, reset_to_zero, dca_mod);

pub struct LfoParams {
  pub shape: ParamBlock,
  pub rate: ParamBlock,
  pub phase: ParamBlock,
  pub depth: ParamBlock,
}

param_blocks!(LfoParams, shape, rate, phase, depth);

pub struct OscParams {
  pub shape: ParamBlock,
  pub amplitude: ParamBlock,
  pub octaves: ParamBlock,
  pub semitones: ParamBlock,
  pub cents: ParamBlock,
}

param_blocks!(OscParams, shape, amplitude, octaves, semitones, cents);

pub struct FilterParams {
  pub mode: ParamBlock,
  pub freq: ParamBlock,
  pub q: ParamBlock,
}

param_blocks!(FilterParams, mode, freq, q);

pub struct DcaParams {
  pub amplitude: ParamBlock,
  pub pan: ParamBlock,
}

param_blocks!(DcaParams, amplitude, pan);
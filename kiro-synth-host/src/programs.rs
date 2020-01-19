use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, Block, osc, ProgramBuilder};
use kiro_synth_midi::messages::Message::ProgramChange;

pub struct Programs;

impl Programs {
  pub fn default<F: Float>() -> Program<F> {
    let mut program = ProgramBuilder::new();

    let zero = program.const_zero();
    let one = program.const_one();

    let note_pitch = ProgramBuilder::<F>::note_pitch();
    let pitch_bend = zero;

    let output_left = ProgramBuilder::<F>::output_left();
    let output_right = ProgramBuilder::<F>::output_right();

    let osc1 = osc::Block {
      inputs: osc::Inputs {
        shape: zero,
        amplitude: one,
        amp_mod: zero,
        octave: zero,
        semitones: zero,
        cents: program.const_value(F::from(-6.0).unwrap()),
        note_pitch,
        pitch_bend,
        freq_mod: one,
      },
      output: program.signal(),
    };

    let osc2 = osc::Block {
      inputs: osc::Inputs {
        shape: zero,
        amplitude: one,
        amp_mod: zero,
        octave: zero,
        semitones: zero,
        cents: program.const_value(F::from(6.0).unwrap()),
        note_pitch,
        pitch_bend,
        freq_mod: one,
      },
      output: program.signal(),
    };

    let out = Block::Out {
      left: osc1.output,
      right: osc2.output,
    };

    program.block(Block::Osc(osc1));
    program.block(Block::Osc(osc2));
    program.block(out);

    program.build()
  }
}

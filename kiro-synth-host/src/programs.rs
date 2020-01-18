use num_traits::Float;
use kiro_synth_engine::program::{Program, Block, osc};
use kiro_synth_midi::messages::Message::ProgramChange;

pub struct Programs;

impl Programs {
  pub fn default<F: Float>() -> Program<F> {
    let mut program = Program::new();

    let zero = program.const_zero();
    let one = program.const_one();

    let note_pitch = Program::<F>::note_pitch();
    let pitch_bend = program.param();

    let output_left = Program::<F>::output_left();
    let output_right = Program::<F>::output_right();

    let osc1 = osc::Block {
      inputs: osc::Inputs {
        shape: program.param(),
        amplitude: program.param(),
        amp_mod: zero,
        octave: program.param(),
        semitones: program.param(),
        cents: program.param(),
        note_pitch,
        pitch_bend,
        freq_mod: zero,
      },
      output: program.signal(),
    };

    let osc2 = osc::Block {
      inputs: osc::Inputs {
        shape: program.param(),
        amplitude: program.param(),
        amp_mod: zero,
        octave: program.param(),
        semitones: program.param(),
        cents: program.param(),
        note_pitch,
        pitch_bend,
        freq_mod: osc1.output,
      },
      output: program.signal(),
    };

    let out = Block::Out {
      left: osc2.output,
      right: osc2.output,
    };

    program.block(Block::Osc(osc1));
    program.block(Block::Osc(osc2));
    program.block(out);

    program
  }
}

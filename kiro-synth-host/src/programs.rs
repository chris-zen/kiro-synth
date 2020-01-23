use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, Block, osc, ProgramBuilder, ParamBlock, SignalRef, ParamValues};


pub struct PlaygroundParams {
  pub pitch_bend: ParamBlock,
  pub osc1_amplitude: ParamBlock,
  pub osc1_octave: ParamBlock,
  pub osc1_semitones: ParamBlock,
  pub osc1_cents: ParamBlock,
  pub osc2_amplitude: ParamBlock,
  pub osc2_octave: ParamBlock,
  pub osc2_semitones: ParamBlock,
  pub osc2_cents: ParamBlock,
}

pub struct PlaygroundSignals {
  pub osc1_output: SignalRef,
  pub osc2_output: SignalRef,
}

pub struct PlaygroundModule {
  pub params: PlaygroundParams,
  pub signals: PlaygroundSignals,
}

impl PlaygroundModule {

  pub fn new_program<'a, F: Float>() -> (Program<'a, F>, PlaygroundModule) {
    let mut program_builder = ProgramBuilder::new();

    let module = Self::new(&mut program_builder);

    program_builder.block(Block::Out {
      left: module.signals.osc1_output,
      right: module.signals.osc2_output,
    });

    (program_builder.build(), module)
  }

  pub fn new<F: Float>(program: &mut ProgramBuilder<F>) -> PlaygroundModule {

    let zero = program.const_zero();

    let note_pitch = ProgramBuilder::<F>::note_pitch();

    let params = PlaygroundParams {
      pitch_bend: program.param("pitch-bend", "Pitch Bend", Self::pitch_bend_values()),
      osc1_amplitude: program.param("osc1-amplitude", "Osc1 Amplitude", Self::amplitude_values()),
      osc1_octave: program.param("osc1-octave", "Osc1 Octave", Self::octave_values()),
      osc1_semitones: program.param("osc1-semitones", "Osc1 Semitones", Self::semitones_values()),
      osc1_cents: program.param("osc1-cents", "Osc1 Cents", Self::cents_values()),
      osc2_amplitude: program.param("osc2-amplitude", "Osc2 Amplitude", Self::amplitude_values()),
      osc2_octave: program.param("osc1-octave", "Osc1 Octave", Self::octave_values()),
      osc2_semitones: program.param("osc1-semitones", "Osc1 Semitones", Self::semitones_values()),
      osc2_cents: program.param("osc1-cents", "Osc1 Cents", Self::cents_values()),
    };

    let signals = PlaygroundSignals {
      osc1_output: program.signal(),
      osc2_output: program.signal(),
    };

    let osc1 = osc::Block {
      inputs: osc::Inputs {
        shape: zero,
        amplitude: params.osc1_amplitude.signal,
        amp_mod: zero,
        octave: params.osc1_octave.signal,
        semitones: params.osc1_semitones.signal,
        cents: params.osc1_cents.signal,
        note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: program.const_value(F::from(0.8).unwrap()),
      },
      output: signals.osc1_output,
    };

    program.block(Block::Osc(osc1));

    let osc2 = osc::Block {
      inputs: osc::Inputs {
        shape: zero,
        amplitude: params.osc2_amplitude.signal,
        amp_mod: zero,
        octave: params.osc2_octave.signal,
        semitones: params.osc2_semitones.signal,
        cents: params.osc2_cents.signal,
        note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: program.const_value(F::from(1.2).unwrap()),
      },
      output: signals.osc2_output,
    };

    program.block(Block::Osc(osc2));

    PlaygroundModule {
      params,
      signals,
    }
  }

  fn pitch_bend_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::one().neg(),
      max: F::one(),
      resolution: F::epsilon(),
    }
  }

  fn amplitude_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::one(),
      min: F::zero(),
      max: F::one(),
      resolution: F::epsilon(),
    }
  }

  fn octave_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::from(-8.0).unwrap(),
      max: F::from(8.0).unwrap(),
      resolution: F::one(),
    }
  }

  fn semitones_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::from(-12.0).unwrap(),
      max: F::from(12.0).unwrap(),
      resolution: F::one(),
    }
  }

  fn cents_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::from(-100.0).unwrap(),
      max: F::from(100.0).unwrap(),
      resolution: F::one(),
    }
  }
}

use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, Block, osc, ProgramBuilder, ParamBlock, SignalRef, ParamValues};
use kiro_synth_engine::program::expr::ExprBuilder;


pub struct PlaygroundParams {
  pub pitch_bend: ParamBlock,

  pub osc1_shape: ParamBlock,
  pub osc1_amplitude: ParamBlock,
  pub osc1_octave: ParamBlock,
  pub osc1_semitones: ParamBlock,
  pub osc1_cents: ParamBlock,

  pub osc2_shape: ParamBlock,
  pub osc2_amplitude: ParamBlock,
  pub osc2_octave: ParamBlock,
  pub osc2_semitones: ParamBlock,
  pub osc2_cents: ParamBlock,

  pub osc3_shape: ParamBlock,
  pub osc3_amplitude: ParamBlock,
  pub osc3_octave: ParamBlock,
  pub osc3_semitones: ParamBlock,
  pub osc3_cents: ParamBlock,

  pub osc4_shape: ParamBlock,
  pub osc4_amplitude: ParamBlock,
  pub osc4_octave: ParamBlock,
  pub osc4_semitones: ParamBlock,
  pub osc4_cents: ParamBlock,
}

pub struct PlaygroundSignals {
  pub osc1_output: SignalRef,
  pub osc2_output: SignalRef,
  pub osc3_output: SignalRef,
  pub osc4_output: SignalRef,
}

pub struct PlaygroundModule {
  pub params: PlaygroundParams,
  pub signals: PlaygroundSignals,
}

impl PlaygroundModule {

  pub fn new_program<'a, F: Float>(num_shapes: usize) -> (Program<'a, F>, PlaygroundModule) {
    let mut program_builder = ProgramBuilder::new();

    let module = Self::new(&mut program_builder, num_shapes);

    let mix_osc3_osc4 = {
      let mut expr = ExprBuilder::new();
      let osc3_output_expr = expr.signal(module.signals.osc3_output);
      let osc4_output_expr = expr.signal(module.signals.osc4_output);
      expr.add(osc3_output_expr, osc4_output_expr);
      expr.build(&mut program_builder)
    };

    program_builder.block(Block::Expr(mix_osc3_osc4.clone()));

    program_builder.block(Block::Out {
      left: mix_osc3_osc4.output,
      right: mix_osc3_osc4.output,
    });

    (program_builder.build(), module)
  }

  pub fn new<F: Float>(program: &mut ProgramBuilder<F>, num_shapes: usize) -> PlaygroundModule {

    let zero = program.const_zero();
    let one = program.const_one();

    let note_pitch = ProgramBuilder::<F>::note_pitch();

    let params = PlaygroundParams {
      pitch_bend: program.param("pitch-bend", "Pitch Bend", Self::pitch_bend_values()),

      osc1_shape: program.param("osc1-shape", "Osc1 Shape", Self::shape_values(num_shapes)),
      osc1_amplitude: program.param("osc1-amplitude", "Osc1 Amplitude", Self::amplitude_values().with_initial_value(F::zero())),
      osc1_octave: program.param("osc1-octave", "Osc1 Octave", Self::octave_values()),
      osc1_semitones: program.param("osc1-semitones", "Osc1 Semitones", Self::semitones_values()),
      osc1_cents: program.param("osc1-cents", "Osc1 Cents", Self::cents_values()),

      osc2_shape: program.param("osc2-shape", "Osc2 Shape", Self::shape_values(num_shapes)),
      osc2_amplitude: program.param("osc2-amplitude", "Osc2 Amplitude", Self::amplitude_values().with_initial_value(F::zero())),
      osc2_octave: program.param("osc2-octave", "Osc2 Octave", Self::octave_values()),
      osc2_semitones: program.param("osc2-semitones", "Osc2 Semitones", Self::semitones_values()),
      osc2_cents: program.param("osc2-cents", "Osc2 Cents", Self::cents_values()),

      osc3_shape: program.param("osc3-shape", "Osc3 Shape", Self::shape_values(num_shapes)),
      osc3_amplitude: program.param("osc3-amplitude", "Osc3 Amplitude", Self::amplitude_values()),
      osc3_octave: program.param("osc3-octave", "Osc3 Octave", Self::octave_values()),
      osc3_semitones: program.param("osc3-semitones", "Osc3 Semitones", Self::semitones_values()),
      osc3_cents: program.param("osc3-cents", "Osc3 Cents", Self::cents_values()),

      osc4_shape: program.param("osc4-shape", "Osc4 Shape", Self::shape_values(num_shapes)),
      osc4_amplitude: program.param("osc4-amplitude", "Osc4 Amplitude", Self::amplitude_values()),
      osc4_octave: program.param("osc4-octave", "Osc4 Octave", Self::octave_values()),
      osc4_semitones: program.param("osc4-semitones", "Osc4 Semitones", Self::semitones_values()),
      osc4_cents: program.param("osc4-cents", "Osc4 Cents", Self::cents_values()),
    };

    let signals = PlaygroundSignals {
      osc1_output: program.signal(),
      osc2_output: program.signal(),
      osc3_output: program.signal(),
      osc4_output: program.signal(),
    };

    let osc1 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc1_shape.signal,
        amplitude: params.osc1_amplitude.signal,
        amp_mod: zero,
        octave: params.osc1_octave.signal,
        semitones: params.osc1_semitones.signal,
        cents: params.osc1_cents.signal,
        note_pitch: program.const_value(F::val(440)),
        pitch_bend: zero,
        freq_mod: zero,
      },
      output: signals.osc1_output,
    };

    let osc2 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc2_shape.signal,
        amplitude: params.osc2_amplitude.signal,
        amp_mod: zero,
        octave: params.osc2_octave.signal,
        semitones: params.osc2_semitones.signal,
        cents: params.osc2_cents.signal,
        note_pitch: program.const_value(F::val(440)),
        pitch_bend: zero,
        freq_mod: zero,
      },
      output: signals.osc2_output,
    };

    let osc3 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc3_shape.signal,
        amplitude: params.osc3_amplitude.signal,
        amp_mod: osc1.output,
        octave: params.osc3_octave.signal,
        semitones: params.osc3_semitones.signal,
        cents: params.osc3_cents.signal,
        note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: zero,
      },
      output: signals.osc3_output,
    };

    let osc4_freq_mod = {
      let mut expr = ExprBuilder::new();
      let osc2_output_expr = expr.signal(signals.osc2_output);
      expr.mul_value(osc2_output_expr, F::val(10.0));
      expr.build(program)
    };

    let osc4 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc4_shape.signal,
        amplitude: params.osc4_amplitude.signal,
        amp_mod: zero,
        octave: params.osc4_octave.signal,
        semitones: params.osc4_semitones.signal,
        cents: params.osc4_cents.signal,
        note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: osc4_freq_mod.output,
      },
      output: signals.osc4_output,
    };

    program.block(Block::Osc(osc1));
    program.block(Block::Osc(osc2));
    program.block(Block::Expr(osc4_freq_mod));
    program.block(Block::Osc(osc3));
    program.block(Block::Osc(osc4));

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

  fn shape_values<F: Float>(num_shapes: usize) -> ParamValues<F> {
    ParamValues {
      initial_value: F::one(),
      min: F::zero(),
      max: F::val(num_shapes - 1),
      resolution: F::one(),
    }
  }

  fn amplitude_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::one(),
      min: F::zero(),
      max: F::one(),
      resolution: F::val(0.01),
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

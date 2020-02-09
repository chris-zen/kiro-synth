use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, Block, ProgramBuilder, ParamBlock, SignalRef, ParamValues, Param};
use kiro_synth_engine::program::{dca, envgen, filter, osc};
use kiro_synth_engine::program::expr::ExprBuilder;
use kiro_synth_core::filters::freq_control::FreqControl;


pub struct PlaygroundParams {
  pub pitch_bend: ParamBlock,

  pub eg1_attack: ParamBlock,
  pub eg1_decay: ParamBlock,
  pub eg1_sustain: ParamBlock,
  pub eg1_release: ParamBlock,
  pub eg1_mode: ParamBlock,
  pub eg1_legato: ParamBlock,
  pub eg1_reset_to_zero: ParamBlock,
  pub eg1_dca_intensity: ParamBlock,

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

  pub filt1_mode: ParamBlock,
  pub filt1_freq: ParamBlock,
  pub filt1_q: ParamBlock,

  pub dca_amplitude: ParamBlock,
  pub dca_pan: ParamBlock,
}

pub struct PlaygroundSignals {
  pub osc1_output: SignalRef,
  pub osc2_output: SignalRef,
  pub osc3_output: SignalRef,
  pub osc4_output: SignalRef,
  pub dca_left: SignalRef,
  pub dca_right: SignalRef,
}

pub struct PlaygroundModule {
  pub params: PlaygroundParams,
  pub signals: PlaygroundSignals,
}

impl PlaygroundModule {

  pub fn new_program<'a, F: Float>(num_shapes: usize) -> (Program<'a, F>, PlaygroundModule) {
    let mut program_builder = ProgramBuilder::new();

    let module = Self::new(&mut program_builder, num_shapes);

    program_builder.out(module.signals.dca_left, module.signals.dca_right);

    (program_builder.build(), module)
  }

  pub fn new<F: Float>(program: &mut ProgramBuilder<F>, num_shapes: usize) -> PlaygroundModule {

    let voice = program.voice().clone();

    let zero = program.const_zero();
    let one = program.const_one();

    let num_filters = filter::Mode::count();

    let params = PlaygroundParams {
      pitch_bend: program.param("pitch-bend", "Pitch Bend", Self::pitch_bend_values()),

      eg1_attack: program.param("eg1-attack", "EG1 Attack", Self::adsr_values(F::val(0.02))),
      eg1_decay: program.param("eg1-decay", "EG1 Decay", Self::adsr_values(F::val(0.1))),
      eg1_sustain: program.param("eg1-sustain", "EG1 Sustain", Self::adsr_values(F::val(0.9))),
      eg1_release: program.param("eg1-release", "EG1 Release", Self::adsr_values(F::val(1.5))),
      eg1_mode: program.param("eg1-mode", "EG1 Mode", Self::mode_values()),
      eg1_legato: program.param("eg1-legato", "EG1 Legato", Self::bool_values(false)),
      eg1_reset_to_zero: program.param("eg1-reset-to-zero", "EG1 Reset To Zero", Self::bool_values(false)),
      eg1_dca_intensity: program.param("eg1-dca-intensity", "EG1-DCA Intensity", Self::intensity_values()),

      osc1_shape: program.param("osc1-shape", "Osc1 Shape", Self::enum_values(num_shapes)),
      osc1_amplitude: program.param("osc1-amplitude", "Osc1 Amplitude", Self::amplitude_values().with_initial_value(F::zero())),
      osc1_octave: program.param("osc1-octave", "Osc1 Octave", Self::octave_values()),
      osc1_semitones: program.param("osc1-semitones", "Osc1 Semitones", Self::semitones_values()),
      osc1_cents: program.param("osc1-cents", "Osc1 Cents", Self::cents_values()),

      osc2_shape: program.param("osc2-shape", "Osc2 Shape", Self::enum_values(num_shapes)),
      osc2_amplitude: program.param("osc2-amplitude", "Osc2 Amplitude", Self::amplitude_values().with_initial_value(F::zero())),
      osc2_octave: program.param("osc2-octave", "Osc2 Octave", Self::octave_values()),
      osc2_semitones: program.param("osc2-semitones", "Osc2 Semitones", Self::semitones_values()),
      osc2_cents: program.param("osc2-cents", "Osc2 Cents", Self::cents_values()),

      osc3_shape: program.param("osc3-shape", "Osc3 Shape", Self::enum_values(num_shapes)),
      osc3_amplitude: program.param("osc3-amplitude", "Osc3 Amplitude", Self::amplitude_values()),
      osc3_octave: program.param("osc3-octave", "Osc3 Octave", Self::octave_values()),
      osc3_semitones: program.param("osc3-semitones", "Osc3 Semitones", Self::semitones_values()),
      osc3_cents: program.param("osc3-cents", "Osc3 Cents", Self::cents_values()),

      osc4_shape: program.param("osc4-shape", "Osc4 Shape", Self::enum_values(num_shapes)),
      osc4_amplitude: program.param("osc4-amplitude", "Osc4 Amplitude", Self::amplitude_values()),
      osc4_octave: program.param("osc4-octave", "Osc4 Octave", Self::octave_values()),
      osc4_semitones: program.param("osc4-semitones", "Osc4 Semitones", Self::semitones_values()),
      osc4_cents: program.param("osc4-cents", "Osc4 Cents", Self::cents_values()),

      filt1_mode: program.param("filt1-mode", "Filt1 Mode", Self::enum_values(num_filters)),
      filt1_freq: program.param("filt1-freq", "Filt1 Frequency", Self::filt_freq_values()),
      filt1_q: program.param("filt1-q", "Filt1 Q", Self::filt_q_values()),

      dca_amplitude: program.param("dca-amplitude-db", "DCA Amplitude dB", Self::amplitude_db_values()),
      dca_pan: program.param("dca-pan", "DCA Pan", Self::pan_values()),
    };

    let signals = PlaygroundSignals {
      osc1_output: program.signal(),
      osc2_output: program.signal(),
      osc3_output: program.signal(),
      osc4_output: program.signal(),
      dca_left: program.signal(),
      dca_right: program.signal(),
    };

    let eg1 = envgen::Block {
      inputs: envgen::Inputs {
        attack: params.eg1_attack.signal,
        decay: params.eg1_decay.signal,
        sustain: params.eg1_sustain.signal,
        release: params.eg1_release.signal,
        mode: params.eg1_mode.signal,
        legato: params.eg1_legato.signal,
        reset_to_zero: params.eg1_reset_to_zero.signal,
      },
      outputs: envgen::Outputs {
        normal: program.signal(),
        biased: program.signal(),
        voice_off: voice.off,
      }
    };

    let eg1_dca_intensity = program.expr(|expr| {
      expr.mul_signal_param(eg1.outputs.normal, params.eg1_dca_intensity.reference)
    });

    let osc1 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc1_shape.signal,
        amplitude: params.osc1_amplitude.signal,
        amp_mod: zero,
        octave: params.osc1_octave.signal,
        semitones: params.osc1_semitones.signal,
        cents: params.osc1_cents.signal,
        note_pitch: program.const_value(F::val(1)),
        pitch_bend: zero,
        freq_mod: zero,
      },
      output: signals.osc1_output,
    };

//    let osc2 = osc::Block {
//      inputs: osc::Inputs {
//        shape: params.osc2_shape.signal,
//        amplitude: params.osc2_amplitude.signal,
//        amp_mod: zero,
//        octave: params.osc2_octave.signal,
//        semitones: params.osc2_semitones.signal,
//        cents: params.osc2_cents.signal,
//        note_pitch: program.const_value(F::val(440)),
//        pitch_bend: zero,
//        freq_mod: zero,
//      },
//      output: signals.osc2_output,
//    };

    let osc3 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc3_shape.signal,
        amplitude: params.osc3_amplitude.signal,
        amp_mod: osc1.output,
        octave: params.osc3_octave.signal,
        semitones: params.osc3_semitones.signal,
        cents: params.osc3_cents.signal,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: zero,
      },
      output: signals.osc3_output,
    };

//    let osc4_freq_mod = {
//      let mut expr = ExprBuilder::new();
//      let osc2_output_expr = expr.signal(signals.osc2_output);
//      expr.mul_value(osc2_output_expr, F::val(10.0));
//      expr.build(program)
//    };

    let osc4 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc4_shape.signal,
        amplitude: params.osc4_amplitude.signal,
        amp_mod: zero,
        octave: params.osc4_octave.signal,
        semitones: params.osc4_semitones.signal,
        cents: params.osc4_cents.signal,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: zero,
      },
      output: signals.osc4_output,
    };

    let osc_mix = program.expr(|expr| {
      expr.add_signals(osc3.output, osc4.output)
    });

    let filt1 = filter::Block {
      input: osc_mix.output,
      params: filter::Params {
        mode: params.filt1_mode.signal,
        freq: params.filt1_freq.signal,
        freq_mod: one,
        q: params.filt1_q.signal,
      },
      output: program.signal(),
    };

    let dca = dca::Block {
      inputs: dca::Inputs {
        left: filt1.output,
        right: filt1.output,
        velocity: voice.velocity,
        amplitude: params.dca_amplitude.signal,
        amp_mod: osc1.output,
        eg_mod: eg1_dca_intensity.output,
        pan: params.dca_pan.signal,
        pan_mod: zero,
      },
      outputs: dca::Outputs {
        left: signals.dca_left,
        right: signals.dca_right,
      }
    };

    program.block(Block::EG(eg1));
    program.block(Block::Expr(eg1_dca_intensity));
    program.block(Block::Osc(osc1));
//    program.block(Block::Osc(osc2));
//    program.block(Block::Expr(osc4_freq_mod));
    program.block(Block::Osc(osc3));
    program.block(Block::Osc(osc4));
    program.block(Block::Expr(osc_mix));
    program.block(Block::Filter(filt1));
    program.block(Block::DCA(dca));

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

  fn adsr_values<F: Float>(value: F) -> ParamValues<F> {
    ParamValues {
      initial_value: value,
      min: F::zero(),
      max: F::val(10.0),
      resolution: F::val(0.01),
    }
  }

  fn mode_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::zero(),
      max: F::one(),
      resolution: F::one(),
    }
  }

  fn bool_values<F: Float>(initial: bool) -> ParamValues<F> {
    ParamValues {
      initial_value: if initial { F::one() } else { F::zero() },
      min: F::zero(),
      max: F::one(),
      resolution: F::one(),
    }
  }

  fn enum_values<F: Float>(count: usize) -> ParamValues<F> {
    ParamValues {
      initial_value: F::val(2), //FIXME 0.0
      min: F::zero(),
      max: F::val(count - 1),
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


  fn amplitude_db_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::val(0.0),
      min: F::val(-96.0),
      max: F::val(24.0),
      resolution: F::val(0.1),
    }
  }

  fn octave_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::val(-8.0),
      max: F::val(8.0),
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
      min: F::val(-100.0),
      max: F::val(100.0),
      resolution: F::one(),
    }
  }

  fn filt_freq_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: FreqControl::default_frequency(),
      min: FreqControl::min_frequency(),
      max: FreqControl::max_frequency(),
      resolution: F::val(10.0),
    }
  }

  fn filt_q_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::val(0.01),
      min: F::zero(),
      max: F::one(),
      resolution: F::val(0.01),
    }
  }

  fn pan_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::zero(),
      min: F::one().neg(),
      max: F::one(),
      resolution: F::val(0.01),
    }
  }

  fn intensity_values<F: Float>() -> ParamValues<F> {
    ParamValues {
      initial_value: F::one(),
      min: F::zero(),
      max: F::val(4.0),
      resolution: F::val(0.01),
    }
  }
}

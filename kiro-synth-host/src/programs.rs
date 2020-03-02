use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, Block, ProgramBuilder, ParamBlock, SignalRef, ParamValues};
use kiro_synth_engine::program::{dca, envgen, filter, osc};
use kiro_synth_core::filters::freq_control::FreqControl;

use crate::program::params::{EnvGenParams, OscParams, FilterParams, DcaParams};

pub struct KiroParams {
  pub pitch_bend: ParamBlock,

  pub eg1: EnvGenParams,

  pub osc1: OscParams,
  pub osc2: OscParams,
  pub osc3: OscParams,
  pub osc4: OscParams,

  pub filt1: FilterParams,

  pub dca: DcaParams,
}

pub struct KiroSignals {
  pub eg1_normal: SignalRef,
  pub eg1_biased: SignalRef,
  pub osc1: SignalRef,
  pub osc2: SignalRef,
  pub osc3: SignalRef,
  pub osc4: SignalRef,
  pub filt1: SignalRef,
  pub dca_left: SignalRef,
  pub dca_right: SignalRef,
}

pub struct KiroModule {
  pub params: KiroParams,
  pub signals: KiroSignals,
}

impl KiroModule {

  pub fn new_program<'a, F: Float>(num_shapes: usize) -> (Program<'a, F>, KiroModule) {
    let mut program_builder = ProgramBuilder::new();

    let module = Self::new(&mut program_builder, num_shapes);

    program_builder.out(module.signals.dca_left, module.signals.dca_right);

    (program_builder.build(), module)
  }

  pub fn new<F: Float>(program: &mut ProgramBuilder<F>, num_shapes: usize) -> KiroModule {

    let voice = program.voice().clone();

    let zero = program.const_zero();
    let one = program.const_one();

    let num_filters = filter::Mode::count();

    let params = KiroParams {
      pitch_bend: program.param("pitch-bend", Self::pitch_bend_values()),

      eg1: EnvGenParams {
        attack: program.param("eg1-attack", Self::adsr_values(F::val(0.02))),
        decay: program.param("eg1-decay", Self::adsr_values(F::val(0.1))),
        sustain: program.param("eg1-sustain", Self::adsr_values(F::val(0.9))),
        release: program.param("eg1-release", Self::adsr_values(F::val(1.5))),
        mode: program.param("eg1-mode", Self::mode_values()),
        legato: program.param("eg1-legato", Self::bool_values(false)),
        reset_to_zero: program.param("eg1-reset-to-zero", Self::bool_values(false)),
        dca_intensity: program.param("eg1-dca-intensity", Self::intensity_values()),
      },

      osc1: OscParams {
        shape: program.param("osc1-shape", Self::enum_values(num_shapes)),
        amplitude: program.param("osc1-amplitude", Self::amplitude_values().with_initial_value(F::zero())),
        octave: program.param("osc1-octave", Self::octave_values()),
        semitones: program.param("osc1-semitones", Self::semitones_values()),
        cents: program.param("osc1-cents", Self::cents_values()),
      },

      osc2: OscParams {
        shape: program.param("osc2-shape", Self::enum_values(num_shapes)),
        amplitude: program.param("osc2-amplitude", Self::amplitude_values().with_initial_value(F::zero())),
        octave: program.param("osc2-octave", Self::octave_values()),
        semitones: program.param("osc2-semitones", Self::semitones_values()),
        cents: program.param("osc2-cents", Self::cents_values()),
      },

      osc3: OscParams {
        shape: program.param("osc3-shape", Self::enum_values(num_shapes)),
        amplitude: program.param("osc3-amplitude", Self::amplitude_values()),
        octave: program.param("osc3-octave", Self::octave_values()),
        semitones: program.param("osc3-semitones", Self::semitones_values()),
        cents: program.param("osc3-cents", Self::cents_values()),
      },

      osc4: OscParams {
        shape: program.param("osc4-shape", Self::enum_values(num_shapes)),
        amplitude: program.param("osc4-amplitude", Self::amplitude_values()),
        octave: program.param("osc4-octave", Self::octave_values()),
        semitones: program.param("osc4-semitones", Self::semitones_values()),
        cents: program.param("osc4-cents", Self::cents_values()),
      },

      filt1: FilterParams {
        mode: program.param("filt1-mode", Self::enum_values(num_filters)),
        freq: program.param("filt1-freq", Self::filt_freq_values()),
        q: program.param("filt1-q", Self::filt_q_values()),
      },

      dca: DcaParams {
        amplitude: program.param("dca-amplitude-db", Self::amplitude_db_values()),
        pan: program.param("dca-pan", Self::pan_values()),
      },
    };

    let signals = KiroSignals {
      eg1_normal: program.signal(),
      eg1_biased: program.signal(),
      osc1: program.signal(),
      osc2: program.signal(),
      osc3: program.signal(),
      osc4: program.signal(),
      filt1: program.signal(),
      dca_left: program.signal(),
      dca_right: program.signal(),
    };

    let eg1 = envgen::Block {
      inputs: envgen::Inputs {
        attack: params.eg1.attack.signal,
        decay: params.eg1.decay.signal,
        sustain: params.eg1.sustain.signal,
        release: params.eg1.release.signal,
        mode: params.eg1.mode.signal,
        legato: params.eg1.legato.signal,
        reset_to_zero: params.eg1.reset_to_zero.signal,
      },
      outputs: envgen::Outputs {
        normal: signals.eg1_normal,
        biased: signals.eg1_biased,
        voice_off: voice.off,
      }
    };

    let eg1_dca_intensity = program.expr(|expr| {
      expr.mul_signal_param(eg1.outputs.normal, params.eg1.dca_intensity.reference)
    });

    // let osc1 = osc::Block {
    //   inputs: osc::Inputs {
    //     shape: params.osc1_shape.signal,
    //     amplitude: params.osc1_amplitude.signal,
    //     amp_mod: zero,
    //     octave: params.osc1_octave.signal,
    //     semitones: params.osc1_semitones.signal,
    //     cents: params.osc1_cents.signal,
    //     note_pitch: program.const_value(F::val(1)),
    //     pitch_bend: zero,
    //     freq_mod: zero,
    //   },
    //   output: signals.osc1,
    // };

    // let osc2 = osc::Block {
    //   inputs: osc::Inputs {
    //     shape: params.osc2_shape.signal,
    //     amplitude: params.osc2_amplitude.signal,
    //     amp_mod: zero,
    //     octave: params.osc2_octave.signal,
    //     semitones: params.osc2_semitones.signal,
    //     cents: params.osc2_cents.signal,
    //     note_pitch: program.const_value(F::val(440)),
    //     pitch_bend: zero,
    //     freq_mod: zero,
    //   },
    //   output: signals.osc2,
    // };

    let osc3 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc3.shape.signal,
        amplitude: params.osc3.amplitude.signal,
        amp_mod: zero,
        octave: params.osc3.octave.signal,
        semitones: params.osc3.semitones.signal,
        cents: params.osc3.cents.signal,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: zero,
      },
      output: signals.osc3,
    };

//    let osc4_freq_mod = {
//      let mut expr = ExprBuilder::new();
//      let osc2_output_expr = expr.signal(signals.osc2_output);
//      expr.mul_value(osc2_output_expr, F::val(10.0));
//      expr.build(program)
//    };

    let osc4 = osc::Block {
      inputs: osc::Inputs {
        shape: params.osc4.shape.signal,
        amplitude: params.osc4.amplitude.signal,
        amp_mod: zero,
        octave: params.osc4.octave.signal,
        semitones: params.osc4.semitones.signal,
        cents: params.osc4.cents.signal,
        note_pitch: voice.note_pitch,
        pitch_bend: params.pitch_bend.signal,
        freq_mod: zero,
      },
      output: signals.osc4,
    };

    let osc_mix = program.expr(|expr| {
      expr.add_signals(osc3.output, osc4.output)
    });

    let filt1 = filter::Block {
      input: osc_mix.output,
      params: filter::Params {
        mode: params.filt1.mode.signal,
        freq: params.filt1.freq.signal,
        freq_mod: one,
        q: params.filt1.q.signal,
      },
      output: signals.filt1,
    };

    let dca = dca::Block {
      inputs: dca::Inputs {
        left: filt1.output,
        right: filt1.output,
        velocity: voice.velocity,
        amplitude: params.dca.amplitude.signal,
        amp_mod: zero,
        eg_mod: eg1_dca_intensity.output,
        pan: params.dca.pan.signal,
        pan_mod: zero,
      },
      outputs: dca::Outputs {
        left: signals.dca_left,
        right: signals.dca_right,
      }
    };

    program.block(Block::EG(eg1));
    program.block(Block::Expr(eg1_dca_intensity));
    // program.block(Block::Osc(osc1));
//    program.block(Block::Osc(osc2));
//    program.block(Block::Expr(osc4_freq_mod));
    program.block(Block::Osc(osc3));
    program.block(Block::Osc(osc4));
    program.block(Block::Expr(osc_mix));
    program.block(Block::Filter(filt1));
    program.block(Block::DCA(dca));

    KiroModule {
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
      initial_value: F::val(0.0),
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

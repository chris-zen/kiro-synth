use kiro_synth_core::float::Float;
use kiro_synth_engine::program::{Program, Block, ProgramBuilder, ParamBlock, SignalRef};
use kiro_synth_engine::program::{dca, envgen, filter, osc};

use crate::program::params::{EnvGenParams, OscParams, FilterParams, DcaParams};
use crate::program::values;

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
      pitch_bend: program.param("pitch-bend", values::pitch_bend()),

      eg1: EnvGenParams {
        attack: program.param("eg1-attack", values::adsr(0.02)),
        decay: program.param("eg1-decay", values::adsr(0.1)),
        sustain: program.param("eg1-sustain", values::adsr(0.9)),
        release: program.param("eg1-release", values::adsr(1.5)),
        mode: program.param("eg1-mode", values::eg_mode()),
        legato: program.param("eg1-legato", values::boolean(false)),
        reset_to_zero: program.param("eg1-reset-to-zero", values::boolean(false)),
        dca_intensity: program.param("eg1-dca-intensity", values::intensity()),
      },

      osc1: OscParams {
        shape: program.param("osc1-shape", values::enumeration(num_shapes)),
        amplitude: program.param("osc1-amplitude", values::amplitude().with_initial_value(F::zero())),
        octaves: program.param("osc1-octaves", values::octave()),
        semitones: program.param("osc1-semitones", values::semitones()),
        cents: program.param("osc1-cents", values::cents()),
      },

      osc2: OscParams {
        shape: program.param("osc2-shape", values::enumeration(num_shapes)),
        amplitude: program.param("osc2-amplitude", values::amplitude().with_initial_value(F::zero())),
        octaves: program.param("osc2-octaves", values::octave()),
        semitones: program.param("osc2-semitones", values::semitones()),
        cents: program.param("osc2-cents", values::cents()),
      },

      osc3: OscParams {
        shape: program.param("osc3-shape", values::enumeration(num_shapes)),
        amplitude: program.param("osc3-amplitude", values::amplitude()),
        octaves: program.param("osc3-octaves", values::octave()),
        semitones: program.param("osc3-semitones", values::semitones()),
        cents: program.param("osc3-cents", values::cents()),
      },

      osc4: OscParams {
        shape: program.param("osc4-shape", values::enumeration(num_shapes)),
        amplitude: program.param("osc4-amplitude", values::amplitude()),
        octaves: program.param("osc4-octaves", values::octave()),
        semitones: program.param("osc4-semitones", values::semitones()),
        cents: program.param("osc4-cents", values::cents()),
      },

      filt1: FilterParams {
        mode: program.param("filt1-mode", values::enumeration(num_filters)),
        freq: program.param("filt1-freq", values::filt_freq()),
        q: program.param("filt1-q", values::filt_q()),
      },

      dca: DcaParams {
        amplitude: program.param("dca-amplitude-db", values::amplitude_db()),
        pan: program.param("dca-pan", values::pan()),
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
        octaves: params.osc3.octaves.signal,
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
        octaves: params.osc4.octaves.signal,
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
}

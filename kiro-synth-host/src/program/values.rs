use kiro_synth_core::float::Float;
use kiro_synth_core::filters::freq_control::FreqControl;
use kiro_synth_engine::program::ParamValues;
use num_traits::ToPrimitive;

pub fn pitch_bend<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::one().neg(),
    max: F::one(),
    resolution: F::epsilon(),
  }
}

pub fn adsr<F: Float, T: ToPrimitive>(value: T) -> ParamValues<F> {
  ParamValues {
    initial_value: F::val(value),
    min: F::zero(),
    max: F::val(10.0),
    resolution: F::val(0.01),
  }
}

pub fn eg_mode<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::zero(),
    max: F::one(),
    resolution: F::one(),
  }
}

pub fn boolean<F: Float>(initial: bool) -> ParamValues<F> {
  ParamValues {
    initial_value: if initial { F::one() } else { F::zero() },
    min: F::zero(),
    max: F::one(),
    resolution: F::one(),
  }
}

pub fn enumeration<F: Float>(count: usize) -> ParamValues<F> {
  ParamValues {
    initial_value: F::val(0.0),
    min: F::zero(),
    max: F::val(count - 1),
    resolution: F::one(),
  }
}

pub fn amplitude<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::one(),
    min: F::zero(),
    max: F::one(),
    resolution: F::val(0.01),
  }
}


pub fn amplitude_db<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::val(0.0),
    min: F::val(-96.0),
    max: F::val(24.0),
    resolution: F::val(0.1),
  }
}

pub fn octave<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::val(-8.0),
    max: F::val(8.0),
    resolution: F::one(),
  }
}

pub fn semitones<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::from(-12.0).unwrap(),
    max: F::from(12.0).unwrap(),
    resolution: F::one(),
  }
}

pub fn cents<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::val(-100.0),
    max: F::val(100.0),
    resolution: F::one(),
  }
}

pub fn lfo_rate<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::one(),
    min: F::zero(),
    max: F::val(20.0),
    resolution: F::val(0.01),
  }
}

pub fn lfo_phase<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::zero(),
    max: F::one(),
    resolution: F::val(1.0 / 8.0),
  }
}

pub fn filt_freq<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: FreqControl::default_frequency(),
    min: FreqControl::min_frequency(),
    max: FreqControl::max_frequency(),
    resolution: F::val(10.0),
  }
}

pub fn filt_q<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::val(0.5),
    min: F::zero(),
    max: F::one(),
    resolution: F::val(0.01),
  }
}

pub fn pan<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::one().neg(),
    max: F::one(),
    resolution: F::val(0.01),
  }
}

pub fn eg1_dca_amp_mod<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::one(),
    min: F::zero(),
    max: F::val(4.0),
    resolution: F::val(0.01),
  }
}

pub fn lfo_osc_pitch_mod<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::one().neg(),
    max: F::one(),
    resolution: F::val(0.01),
  }
}

pub fn lfo_filt_cutoff_mod<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::one().neg(),
    max: F::one(),
    resolution: F::val(0.01),
  }
}

pub fn lfo_dca_amp_mod<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::val(0.0),
    min: F::zero(),
    max: F::val(24.0),
    resolution: F::val(0.1),
  }
}

pub fn lfo_dca_pan_mod<F: Float>() -> ParamValues<F> {
  ParamValues {
    initial_value: F::zero(),
    min: F::zero(),
    max: F::one(),
    resolution: F::val(0.01),
  }
}

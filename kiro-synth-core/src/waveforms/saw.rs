use num_traits::float::Float;

use crate::blep::{PolyBLEP, TableBLEP, BLEP, BLEP_8_BLACKMAN_HARRIS};
use crate::utils::unipolar_to_bipolar;
use crate::waveforms::Waveform;

/// 8-point BLEP can only be calculated when freq <= Nyquist4, where Nyquist4 is sample_rate / 8
/// Given that the phase_inc is freq / sample_rate, then the maximum phase_inc allowed is 1 / 8
const MAX_PHASE_INC_FOR_8_BLEP: f32 = 1.0 / 8.0;

const DEFAULT_SATURATION: f32 = 1.5;

#[derive(Debug, Clone)]
pub enum Mode {
  Normal,
  Unipolar,
  Bipolar,
}

#[derive(Debug, Clone)]
pub enum Correction {
  None,
  TwoPointBlep,
  TwoPointBlepWithInterpolation,
  EightPointBlep,
  EightPointBlepWithInterpolation,
  PolyBlep,
}

#[derive(Debug, Clone)]
pub struct Saw<F: Float> {
  mode: Mode,
  correction: Correction,
  saturation: F,
}

impl<F: Float> Saw<F> {

  pub fn default_saturation() -> F {
    F::from(DEFAULT_SATURATION).unwrap()
  }

  pub fn new(mode: Mode, correction: Correction, saturation: F) -> Self {
    Saw {
      mode,
      correction,
      saturation,
    }
  }

  pub fn set_saturation(&mut self, saturation: F) {
    self.saturation = saturation;
  }
}

impl<F> Waveform<F> for Saw<F>
where
  F: Float,
{
  fn generate(&mut self, modulo: F, phase_inc: F) -> F {
    let signal = match self.mode {
      Mode::Normal => unipolar_to_bipolar(modulo),
      Mode::Unipolar => {
        unipolar_to_bipolar((self.saturation * modulo).tanh() / self.saturation.tanh())
      }
      Mode::Bipolar => {
        (self.saturation * unipolar_to_bipolar(modulo)).tanh() / self.saturation.tanh()
      }
    };

    let residual = match self.correction {
      Correction::None => F::zero(),
      Correction::TwoPointBlep => BLEP.residual(modulo, phase_inc.abs(), F::one(), false, 1, false),
      Correction::TwoPointBlepWithInterpolation => {
        BLEP.residual(modulo, phase_inc.abs(), F::one(), false, 1, true)
      }
      Correction::EightPointBlep => {
        if phase_inc <= F::from(MAX_PHASE_INC_FOR_8_BLEP).unwrap() {
          BLEP_8_BLACKMAN_HARRIS.residual(modulo, phase_inc.abs(), F::one(), false, 4, false)
        } else {
          BLEP.residual(modulo, phase_inc.abs(), F::one(), false, 1, false)
        }
      }
      Correction::EightPointBlepWithInterpolation => {
        if phase_inc <= F::from(MAX_PHASE_INC_FOR_8_BLEP).unwrap() {
          BLEP_8_BLACKMAN_HARRIS.residual(modulo, phase_inc.abs(), F::one(), false, 4, true)
        } else {
          BLEP.residual(modulo, phase_inc.abs(), F::one(), false, 1, true)
        }
      }
      Correction::PolyBlep => PolyBLEP::residual(modulo, phase_inc.abs(), F::one(), false),
    };

    signal + residual
  }
}

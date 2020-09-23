use crate::float::Float;
use crate::waveforms::Waveform;

#[derive(Debug, Clone)]
pub struct SquareTrivial<F: Float> {
  /// pulse width between [0.0, 1.0)
  pulse_width: F,
}

impl<F: Float> Default for SquareTrivial<F> {
  fn default() -> Self {
    SquareTrivial {
      pulse_width: F::val(0.5),
    }
  }
}

impl<F: Float> SquareTrivial<F> {
  /// pulse width between [0.0, 1.0)
  pub fn new(pulse_width: F) -> Self {
    SquareTrivial { pulse_width }
  }

  /// pulse width between [0.0, 1.0)
  pub fn with_pulse_width(self, pulse_width: F) -> Self {
    SquareTrivial { pulse_width }
  }
}

impl<F: Float> Waveform<F> for SquareTrivial<F> {
  fn generate(&mut self, modulo: F, _phase_inc: F) -> F {
    if modulo <= self.pulse_width {
      F::one()
    } else {
      F::one().neg()
    }
  }
}

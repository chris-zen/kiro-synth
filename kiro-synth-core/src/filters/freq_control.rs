use crate::float::Float;

#[derive(Debug)]
pub struct FreqControl<F: Float> {
  freq: F,
  modulation: F,
  invalidated: bool,
}

impl<F: Float> FreqControl<F> {
  pub fn new(freq: F) -> Self {
    FreqControl {
      freq,
      modulation: F::zero(),
      invalidated: true,
    }
  }

  pub fn set_frequency(&mut self, freq: F) {
    self.freq = freq;
    self.invalidated = true;
  }

  /// Modulation in semitones
  pub fn set_semitones_modulation(&mut self, modulation: F) {
    self.invalidated = true;
    self.modulation = if modulation == F::zero() {
      F::one()
    }
    else {
      F::val(2.0).powf(modulation / F::val(12.0))
    };
  }

  pub fn is_invalidated(&self) -> bool {
    self.invalidated
  }

  pub fn get_modulated_freq(&mut self) -> F {
    self.invalidated = false;
    let fc = self.freq * self.modulation;
    fc.max(Self::min_frequency()).min(Self::max_frequency())
  }

  #[inline]
  pub fn max_frequency() -> F {
    F::val(18_000)
  }

  #[inline]
  pub fn min_frequency() -> F {
    F::val(80)
  }

  #[inline]
  pub fn default_frequency() -> F {
    F::val(1_000)
  }
}

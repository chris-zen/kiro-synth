use crate::float::Float;

/// Frequency Linear Modulation
/// Used with Frequency and Phase modulation (FM/PM)
///
pub struct OscFreqLinearMod<F: Float> {
  ratio: F,
  modulation: F,
}

impl<F> Default for OscFreqLinearMod<F>
where
  F: Float,
{
  fn default() -> Self {
    OscFreqLinearMod {
      ratio: F::one(),
      modulation: F::one(),
    }
  }
}

impl<F> OscFreqLinearMod<F>
where
  F: Float,
{
  /// set frequency ratio
  pub fn set_ratio(&mut self, ratio: F) {
    self.ratio = ratio;
  }

  /// set frequency linear modulation
  pub fn set_modulation(&mut self, modulation: F) {
    self.modulation = modulation;
  }

  pub fn apply(&self, freq: F) -> F {
    // 20.480kHz = 10 octaves up from 20Hz
    let max_freq = F::from(20480.0).unwrap();

    freq
      .mul_add(self.ratio, self.modulation)
      .min(max_freq)
      .max(max_freq.neg())
  }
}

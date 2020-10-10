use crate::float::Float;
use crate::funcs::decibels::Decibels;

pub struct PeakMeter<F> {
  max_peak: F,
  peak: F,
  level: F,
  initial_hold_samples: usize,
  remaining_hold_samples: usize,
  decay_per_sample: F,
}

impl<F: Float> PeakMeter<F> {
  pub fn new(sample_rate: F, hold_seconds: F, decay_db_per_second: F) -> Self {
    let hold_samples = (sample_rate * hold_seconds).round().to_usize().unwrap();
    let decay_per_sample = decay_db_per_second / sample_rate;
    PeakMeter {
      max_peak: F::neg_infinity(),
      peak: F::neg_infinity(),
      level: F::neg_infinity(),
      initial_hold_samples: hold_samples,
      remaining_hold_samples: hold_samples,
      decay_per_sample,
    }
  }

  pub fn reset_all(&mut self) {
    self.max_peak = F::neg_infinity();
    self.peak = F::neg_infinity();
    self.level = F::neg_infinity();
    self.remaining_hold_samples = self.initial_hold_samples;
  }

  pub fn reset_peak_max(&mut self) {
    self.max_peak = F::neg_infinity();
  }

  pub fn process(&mut self, value: F) {
    let value = Decibels::from_amplitude(value).value();

    self.max_peak = self.max_peak.max(value);

    if value >= self.peak {
      self.peak = value;
      self.remaining_hold_samples = self.initial_hold_samples;
    } else if self.remaining_hold_samples == 0 {
      self.peak = self.peak - self.decay_per_sample;
    } else {
      self.remaining_hold_samples -= 1;
    }

    if value >= self.level {
      self.level = value;
    } else {
      self.level = self.level - self.decay_per_sample;
    }
  }

  pub fn get_max_peak(&self) -> F {
    self.max_peak
  }

  pub fn get_peak(&self) -> F {
    self.peak
  }

  pub fn get_level(&self) -> F {
    self.level
  }
}

#[cfg(test)]
mod test {
  #![allow(clippy::float_cmp)]

  use super::PeakMeter;
  use assert_approx_eq::assert_approx_eq;

  #[test]
  fn test_new() {
    let meter = PeakMeter::<f32>::new(44100.0, 0.8, 12.0);
    assert_eq!(meter.max_peak, f32::NEG_INFINITY);
    assert_eq!(meter.peak, f32::NEG_INFINITY);
    assert_eq!(meter.initial_hold_samples, 35280);
    assert_eq!(meter.remaining_hold_samples, 35280);
    assert_approx_eq!(meter.decay_per_sample, 0.00027210885);
  }

  #[test]
  fn test_reset_all() {
    let mut meter = PeakMeter::<f32>::new(44100.0, 0.8, 12.0);
    (0..10).for_each(|i| meter.process(0.1 * i as f32));
    meter.reset_all();
    assert_eq!(meter.max_peak, f32::NEG_INFINITY);
    assert_eq!(meter.peak, f32::NEG_INFINITY);
    assert_eq!(meter.remaining_hold_samples, 35280);
    assert_approx_eq!(meter.decay_per_sample, 0.00027210885);
  }

  #[test]
  fn test_reset_static_peak() {
    let mut meter = PeakMeter::<f32>::new(44100.0, 0.8, 12.0);
    (0..10).for_each(|i| meter.process(0.1 * i as f32));
    meter.reset_peak_max();
    assert_eq!(meter.max_peak, f32::NEG_INFINITY);
    assert_approx_eq!(meter.peak, -0.91514945);
  }

  #[test]
  fn test_process_max_peak() {
    let mut meter = PeakMeter::<f32>::new(44100.0, 0.8, 12.0);
    meter.process(0.1);
    meter.process(0.3);

    assert_approx_eq!(meter.max_peak, -10.457574);

    (0..100).for_each(|_| meter.process(0.0));

    assert_approx_eq!(meter.max_peak, -10.457574);
  }

  #[test]
  fn test_process_peak() {
    let mut meter = PeakMeter::<f32>::new(44100.0, 0.8, 12.0);
    meter.process(0.1);
    meter.process(0.3);

    assert_approx_eq!(meter.peak, -10.457574);
    assert_eq!(meter.remaining_hold_samples, 35280);

    (0..35280).for_each(|_| meter.process(0.0));

    assert_approx_eq!(meter.peak, -10.457574);
    assert_eq!(meter.remaining_hold_samples, 0);

    meter.process(0.0);

    assert_approx_eq!(meter.peak, -10.457846);
    assert_eq!(meter.remaining_hold_samples, 0);

    (0..1000).for_each(|_| meter.process(0.0));

    assert_approx_eq!(meter.peak, -10.729643);
  }

  #[test]
  fn test_get_peaks() {
    let mut meter = PeakMeter::<f32>::new(44100.0, 0.8, 12.0);
    (0..10).for_each(|i| meter.process(0.1 * i as f32));
    assert_approx_eq!(meter.get_max_peak(), -0.91514945);
    assert_approx_eq!(meter.get_peak(), -0.91514945);
  }
}

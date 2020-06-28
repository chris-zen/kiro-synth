use crate::filters::freq_control::FreqControl;
use crate::filters::q_control::QControl;
use crate::filters::saturation::Saturation;
use crate::float::Float;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
  LowPass,
  HighPass,
  BandPass,
  BandSum,
}

#[derive(Debug)]
pub struct OberheimSEM<F: Float> {
  sample_rate: F,
  inv_sample_rate: F,
  mode: Mode,
  freq: FreqControl<F>,
  q: QControl<F>,
  saturation: Saturation<F>,
  alpha: F,
  alpha0: F,
  rho: F,
  bsf: F,
  z11: F,
  z12: F,
}

impl<F: Float> OberheimSEM<F> {
  pub fn new(sample_rate: F, fc: F, q: F) -> Self {
    OberheimSEM {
      sample_rate,
      inv_sample_rate: F::one() / sample_rate,
      mode: Mode::LowPass,
      freq: FreqControl::new(fc),
      q: QControl::new(F::val(0.5), F::val(25), q),
      saturation: Saturation::new(false),
      alpha: F::one(),
      alpha0: F::one(),
      rho: F::one(),
      bsf: F::val(0.5),
      z11: F::zero(),
      z12: F::zero(),
    }
  }

  pub fn set_mode(&mut self, mode: Mode) {
    self.mode = mode;
  }

  pub fn set_frequency(&mut self, freq: F) {
    self.freq.set_frequency(freq);
  }

  pub fn set_frequency_modulation(&mut self, semitones: F) {
    self.freq.set_semitones_modulation(semitones);
  }

  pub fn set_q(&mut self, q: F) {
    self.q.set_value(q);
  }

  pub fn reset(&mut self) {
    self.z11 = F::zero();
    self.z12 = F::zero();
  }

  pub fn update(&mut self) {
    if self.freq.is_invalidated() || self.q.is_invalidated() {
      let two = F::val(2.0);
      let fc = self.freq.get_modulated_freq();
      let wd = two * F::PI * fc;
      let t = self.inv_sample_rate;
      let wa = (two / t) * (wd * t / two).tan();
      let g = wa * t / two;

      let r = F::one() / (two * self.q.get_scaled_value());

      self.alpha0 = F::one() / (F::one() + two * r * g + g * g);
      self.alpha = g;
      self.rho = two * r + g;
    }
  }

  pub fn process(&mut self, input: F) -> F {
    self.update();

    let hpf = self.alpha0 * (input - self.rho * self.z11 - self.z12);
    let bpf = self.saturation.saturate(self.alpha.mul_add(hpf, self.z11));
    let lpf = self.alpha.mul_add(bpf, self.z12);
    let bsf = self.bsf * hpf + (F::one() - self.bsf) * lpf;

    self.z11 = self.alpha.mul_add(hpf, bpf);
    self.z12 = self.alpha.mul_add(bpf, lpf);

    match self.mode {
      Mode::LowPass => lpf,
      Mode::HighPass => hpf,
      Mode::BandPass => bpf,
      Mode::BandSum => bsf,
    }
  }
}

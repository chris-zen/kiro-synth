use crate::filters::freq_control::FreqControl;
use crate::float::Float;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
  LowPass,
  HighPass,
}

#[derive(Debug)]
pub struct VAOnePoleFilter<F: Float> {
  sample_rate: F,
  inv_sample_rate: F,
  mode: Mode,
  freq: FreqControl<F>,
  alpha: F,
  beta: F,
  gamma: F,
  delta: F,
  epsilon: F,
  a0: F,
  feedback: F,
  z1: F,
}

impl<F: Float> VAOnePoleFilter<F> {
  pub fn new(sample_rate: F, fc: F) -> Self {
    VAOnePoleFilter {
      sample_rate,
      inv_sample_rate: F::one() / sample_rate,
      mode: Mode::LowPass,
      freq: FreqControl::new(fc),
      alpha: F::one(),
      beta: F::zero(),
      z1: F::zero(),
      gamma: F::one(),
      delta: F::zero(),
      epsilon: F::zero(),
      a0: F::one(),
      feedback: F::zero(),
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

  pub fn set_feedback_in(&mut self, feedback: F) {
    self.feedback = feedback;
  }

  pub fn get_feedback_out(&self) -> F {
    self.beta * (self.z1 + self.feedback * self.delta)
  }

  pub fn reset(&mut self) {
    self.z1 = F::zero();
    self.feedback = F::zero();
  }

  pub fn update(&mut self) {
    if self.freq.is_invalidated() {
      let two = F::val(2.0);
      let fc = self.freq.get_modulated_freq();
      let wd = two * F::PI * fc;
      let half_inv_sample_rate = self.inv_sample_rate / two;
      let wa = (two * self.sample_rate) * (wd * half_inv_sample_rate).tan();
      let g = wa * half_inv_sample_rate;
      self.alpha = g / (F::one() + g);
    }
  }

  pub fn process(&mut self, input: F) -> F {
    self.update();

    let xn = input * self.gamma + self.feedback + self.epsilon * self.get_feedback_out();
    let vn = (xn * self.a0 - self.z1) * self.alpha;
    let lpf = vn + self.z1;
    self.z1 = vn + lpf;
    match self.mode {
      Mode::LowPass => lpf,
      Mode::HighPass => xn - lpf,
    }
  }
}

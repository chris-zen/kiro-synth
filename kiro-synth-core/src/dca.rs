use crate::float::Float;
use crate::funcs::decibels::Decibels;

#[derive(Debug, Default)]
pub struct DCA<F: Float> {
  amplitude: F,
  velocity: F,
  amp_mod: F,
  eg_mod: F,

  gain: F,
  gain_invalidated: bool,

  pan: F,
  pan_mod: F,

  pan_left: F,
  pan_right: F,
  pan_invalidated: bool,
}

impl<F: Float> DCA<F> {
  pub fn new() -> Self {
    DCA {
      amplitude: F::one(),
      velocity: F::one(),
      amp_mod: F::zero(),
      eg_mod: F::zero(),
      gain: F::zero(),
      gain_invalidated: true,
      pan: F::zero(),
      pan_mod: F::zero(),
      pan_left: F::val(0.776),
      pan_right: F::val(0.776),
      pan_invalidated: true,
    }
  }

  /// value in decibels
  pub fn set_amplitude_db(&mut self, decibels: F) {
    self.amplitude = Decibels::new(decibels).to_amplitude();
    self.gain_invalidated = true;
  }

  /// value expected to be between 0.0 and 1.0
  pub fn set_velocity(&mut self, value: F) {
    self.velocity = value * value;
    self.gain_invalidated = true;
  }

  /// amount in decibels
  pub fn set_amp_mod_db(&mut self, decibels: F) {
    // I don't think this makes any sense here: let value = bipolar_to_unipolar(decibels);
    self.amp_mod = Decibels::new(decibels).to_amplitude();
    self.gain_invalidated = true;
  }

  /// value expected to be between 0.0 and 1.0
  pub fn set_eg_mod(&mut self, value: F) {
    self.eg_mod = value;
    self.gain_invalidated = true;
  }

  /// value expected to be between -1.0 and 1.0
  pub fn set_pan(&mut self, value: F) {
    self.pan = value;
    self.pan_invalidated = true;
  }

  /// value expected to be between -1.0 and 1.0
  pub fn set_pan_mod(&mut self, value: F) {
    self.pan_mod = value;
    self.pan_invalidated = true;
  }

  pub fn process(&mut self, left: F, right: F) -> (F, F) {
    self.update_gain();
    self.update_pan();

    let left_out = left * self.gain * self.pan_left;
    let right_out = right * self.gain * self.pan_right;
    (left_out, right_out)
  }

  fn update_gain(&mut self) {
    if self.gain_invalidated {
      self.gain_invalidated = false;
      let eg_mod = if self.eg_mod >= F::zero() {
        self.eg_mod
      } else {
        self.eg_mod + F::one()
      };

      self.gain = self.velocity * self.amplitude * self.amp_mod * eg_mod;
      // println!("gain = {:?}, {:?}, {:?}, {:?}, {:?}", self.gain, self.velocity, self.amplitude, self.amp_mod, eg_mod);
    }
  }

  fn update_pan(&mut self) {
    if self.pan_invalidated {
      self.pan_invalidated = false;
      let pan_total = (self.pan + self.pan_mod).max(F::one().neg()).min(F::one());

      let pi_over_four = F::PI / F::val(4.0);
      let pan_left = (pi_over_four * (pan_total + F::one())).cos();
      let pan_right = (pi_over_four * (pan_total + F::one())).sin();

      self.pan_left = pan_left.max(F::zero()).min(F::one());
      self.pan_right = pan_right.max(F::zero()).min(F::one());
      // println!("pan = {:?}, {:?}", self.pan_left, self.pan_right);
    }
  }
}

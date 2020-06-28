use crate::float::Float;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
  Analog,
  Digital,
}

#[derive(Debug, Clone, Copy)]
enum State {
  Off,
  Attack,
  Decay,
  Sustain,
  Release,
  Shutdown,
}

#[derive(Debug, Clone, Copy)]
struct ADR<F: Float> {
  time_sec: F,
  time_constant_overshoot: F,
  coefficient: F,
  offset: F,
}

impl<F: Float> ADR<F> {
  const ANALOG_DECAY_EXPONENT: f32 = -4.95;
  const DIGITAL_DECAY_EXPONENT: f32 = -11.05;

  pub fn attack(sample_rate: F, mode: Mode, time_sec: F) -> ADR<F> {
    let time_constant_overshoot = match mode {
      Mode::Analog => F::val(-1.5).exp(),
      Mode::Digital => F::val(0.99999).exp(),
    };

    let samples = Self::samples(sample_rate, time_sec);
    let tco_plus_one = F::one() + time_constant_overshoot;
    let coefficient = ((tco_plus_one / time_constant_overshoot).ln().neg() / samples).exp();
    let offset = tco_plus_one * (F::one() - coefficient);

    ADR {
      time_sec,
      time_constant_overshoot,
      coefficient,
      offset,
    }
  }

  pub fn decay(sample_rate: F, mode: Mode, time_sec: F, sustain_level: F) -> ADR<F> {
    let time_constant_overshoot = match mode {
      Mode::Analog => F::val(Self::ANALOG_DECAY_EXPONENT).exp(),
      Mode::Digital => F::val(Self::DIGITAL_DECAY_EXPONENT).exp(),
    };

    let samples = Self::samples(sample_rate, time_sec);
    let tco_plus_one = F::one() + time_constant_overshoot;
    let coefficient = ((tco_plus_one / time_constant_overshoot).ln().neg() / samples).exp();
    let offset = (sustain_level - time_constant_overshoot) * (F::one() - coefficient);

    ADR {
      time_sec,
      time_constant_overshoot,
      coefficient,
      offset,
    }
  }

  pub fn release(sample_rate: F, mode: Mode, time_sec: F) -> ADR<F> {
    let time_constant_overshoot = match mode {
      Mode::Analog => F::val(Self::ANALOG_DECAY_EXPONENT).exp(),
      Mode::Digital => F::val(Self::DIGITAL_DECAY_EXPONENT).exp(),
    };

    let samples = Self::samples(sample_rate, time_sec);
    let tco_plus_one = F::one() + time_constant_overshoot;
    let coefficient = ((tco_plus_one / time_constant_overshoot).ln().neg() / samples).exp();
    let offset = time_constant_overshoot.neg() * (F::one() - coefficient);

    ADR {
      time_sec,
      time_constant_overshoot,
      coefficient,
      offset,
    }
  }

  fn samples(sample_rate: F, time_sec: F) -> F {
    sample_rate * time_sec
  }
}

#[derive(Debug, Clone)]
pub struct EnvGen<F: Float> {
  sample_rate: F,

  reset_to_zero: bool,
  legato: bool,
  mode: Mode,

  attack: ADR<F>,
  decay: ADR<F>,
  release: ADR<F>,
  sustain_level: F,
  shutdown_dec: F,

  state: State,
  output: F,
}

impl<F: Float> EnvGen<F> {
  pub fn new(sample_rate: F) -> Self {
    let mode = Mode::Analog;
    let (attack_time_ms, decay_time_ms, release_time_ms) = Self::default_times_sec();
    let sustain_level = F::one();
    EnvGen {
      sample_rate,
      reset_to_zero: false,
      legato: false,
      mode,
      attack: ADR::attack(sample_rate, mode, attack_time_ms),
      decay: ADR::decay(sample_rate, mode, decay_time_ms, sustain_level),
      release: ADR::release(sample_rate, mode, release_time_ms),
      sustain_level,
      shutdown_dec: F::zero(),
      state: State::Off,
      output: F::zero(),
    }
  }

  pub fn set_mode(&mut self, mode: Mode) {
    self.mode = mode;
    self.attack = ADR::attack(self.sample_rate, mode, self.attack.time_sec);
    self.decay = ADR::decay(
      self.sample_rate,
      mode,
      self.decay.time_sec,
      self.sustain_level,
    );
    self.release = ADR::release(self.sample_rate, mode, self.release.time_sec);
  }

  pub fn set_attack_time_sec(&mut self, time_sec: F) {
    self.attack = ADR::attack(self.sample_rate, self.mode, time_sec);
  }

  pub fn set_decay_time_sec(&mut self, time_sec: F) {
    self.decay = ADR::decay(self.sample_rate, self.mode, time_sec, self.sustain_level);
  }

  pub fn set_release_time_sec(&mut self, time_sec: F) {
    self.release = ADR::release(self.sample_rate, self.mode, time_sec);
  }

  pub fn set_sustain_level(&mut self, level: F) {
    self.sustain_level = level;
    self.decay = ADR::decay(
      self.sample_rate,
      self.mode,
      self.decay.time_sec,
      self.sustain_level,
    );
    match self.state {
      State::Release => {}
      _ => self.release = ADR::release(self.sample_rate, self.mode, self.release.time_sec), // TODO guess why needed
    }
  }

  pub fn get_sustain_level(&self) -> F {
    self.sustain_level
  }

  pub fn reset(&mut self) {
    self.state = State::Off;
    self.set_mode(self.mode); // FIXME needed ???
    if self.reset_to_zero {
      self.output = F::zero();
    }
  }

  pub fn start(&mut self) {
    if !self.legato || !self.is_active() {
      self.reset();
      self.state = State::Attack;
    }
  }

  //  pub fn stop(&mut self) {
  //    self.state = State::Off;
  //  }

  pub fn is_active(&self) -> bool {
    match self.state {
      State::Off | State::Release => false,
      _ => true,
    }
  }

  pub fn is_off(&self) -> bool {
    match self.state {
      State::Off => true,
      _ => false,
    }
  }

  pub fn note_off(&mut self) {
    self.state = if self.output > F::zero() {
      State::Release
    } else {
      State::Off
    }
  }

  pub fn shutdown(&mut self) {
    if !self.legato {
      self.shutdown_dec = self.output / (Self::shutdown_time_sec() * self.sample_rate);
      self.state = State::Shutdown;
    }
  }

  pub fn generate(&mut self) -> F {
    match self.state {
      State::Off => {
        if self.reset_to_zero {
          self.output = F::zero();
        }
      }
      State::Attack => {
        self.output = self.attack.offset + self.output * self.attack.coefficient;
        if self.output >= F::one() || self.attack.time_sec <= F::zero() {
          self.output = F::one();
          self.state = State::Decay;
        }
      }
      State::Decay => {
        self.output = self.decay.offset + self.output * self.decay.coefficient;
        if self.output <= self.sustain_level || self.decay.time_sec <= F::zero() {
          self.output = self.sustain_level;
          self.state = State::Sustain;
        }
      }
      State::Sustain => {
        self.output = self.sustain_level;
      }
      State::Release => {
        self.output = self.release.offset + self.output * self.release.coefficient;
        if self.output <= F::zero() || self.release.time_sec <= F::zero() {
          self.output = F::zero();
          self.state = State::Off;
        }
      }
      State::Shutdown => {
        if self.reset_to_zero {
          self.output = self.output - self.shutdown_dec;
          if self.output <= F::zero() {
            self.output = F::zero();
            self.state = State::Off;
          }
        } else {
          self.state = State::Off;
        }
      }
    };
    //    println!("{:?} {:?}", self.state, self.output);
    self.output
  }

  pub fn biased_output(&self) -> F {
    self.output - self.sustain_level
  }

  #[inline]
  fn default_times_sec() -> (F, F, F) {
    (
      F::val(0.2), // attack
      F::val(0.2), // decay
      F::val(1.0), // release
    )
  }

  #[inline]
  fn shutdown_time_sec() -> F {
    F::val(0.01)
  }
}

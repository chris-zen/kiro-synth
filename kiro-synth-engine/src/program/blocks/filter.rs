use kiro_synth_core::filters::freq_control::FreqControl;
use kiro_synth_core::filters::oberheim_sem::{self, OberheimSEM};
use kiro_synth_core::filters::va_one_pole::{self, VAOnePoleFilter};
use kiro_synth_core::float::Float;

use crate::program::{Program, SignalRef};
use crate::signal::SignalBus;
use kiro_synth_core::filters::q_control::QControl;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
  PassThrough,
  VAOnePole(va_one_pole::Mode),
  OberheimSEM(oberheim_sem::Mode),
}

impl Mode {
  const MODES: [Mode; 7] = [
    Mode::PassThrough,
    Mode::VAOnePole(va_one_pole::Mode::LowPass),
    Mode::VAOnePole(va_one_pole::Mode::HighPass),
    Mode::OberheimSEM(oberheim_sem::Mode::LowPass),
    Mode::OberheimSEM(oberheim_sem::Mode::HighPass),
    Mode::OberheimSEM(oberheim_sem::Mode::BandPass),
    Mode::OberheimSEM(oberheim_sem::Mode::BandSum),
  ];

  pub fn count() -> usize {
    Self::MODES.len()
  }

  pub fn from<F: Float>(value: F) -> Option<Self> {
    value
      .to_usize()
      .and_then(|index| Self::MODES.get(index).copied())
  }
}

#[derive(Debug, Clone)]
pub struct Params {
  pub mode: SignalRef,
  pub freq: SignalRef,
  pub freq_mod: SignalRef,
  pub q: SignalRef,
}

#[derive(Debug, Clone)]
pub struct Block {
  pub input: SignalRef,
  pub params: Params,
  pub output: SignalRef,
}

#[derive(Debug)]
pub(crate) struct Processor<F: Float> {
  mode: Mode,
  va_one_pole: VAOnePoleFilter<F>,
  oberheim_sem: OberheimSEM<F>,
  block: Block,
}

impl<F: Float> Processor<F> {
  pub fn new(sample_rate: F, block: Block) -> Self {
    Processor {
      mode: Mode::PassThrough,
      va_one_pole: VAOnePoleFilter::new(sample_rate, FreqControl::default_frequency()),
      oberheim_sem: OberheimSEM::new(
        sample_rate,
        FreqControl::default_frequency(),
        QControl::default_q(),
      ),
      block,
    }
  }

  fn set_mode(&mut self, mode: F) {
    Mode::from(mode.round()).iter().for_each(|mode| {
      self.mode = *mode;
      match self.mode {
        Mode::PassThrough => {}
        Mode::VAOnePole(va_one_pole_mode) => self.va_one_pole.set_mode(va_one_pole_mode),
        Mode::OberheimSEM(oberheim_sem_mode) => self.oberheim_sem.set_mode(oberheim_sem_mode),
      }
    });
  }

  fn set_freq(&mut self, freq: F) {
    match self.mode {
      Mode::PassThrough => {}
      Mode::VAOnePole(_) => self.va_one_pole.set_frequency(freq),
      Mode::OberheimSEM(_) => self.oberheim_sem.set_frequency(freq),
    }
  }

  fn set_freq_mod(&mut self, freq_mod: F) {
    match self.mode {
      Mode::PassThrough => {}
      Mode::VAOnePole(_) => self.va_one_pole.set_frequency_modulation(freq_mod),
      Mode::OberheimSEM(_) => self.oberheim_sem.set_frequency_modulation(freq_mod),
    }
  }

  fn set_q(&mut self, q: F) {
    match self.mode {
      Mode::PassThrough => {}
      Mode::VAOnePole(_) => {}
      Mode::OberheimSEM(_) => self.oberheim_sem.set_q(q),
    }
  }

  pub fn reset(&mut self) {
    match self.mode {
      Mode::PassThrough => {}
      Mode::VAOnePole(_) => self.va_one_pole.reset(),
      Mode::OberheimSEM(_) => self.oberheim_sem.reset(),
    }
  }

  pub fn process<'a>(&mut self, signals: &mut SignalBus<'a, F>, _program: &Program<F>) {
    let Params {
      mode,
      freq,
      freq_mod,
      q,
    } = self.block.params;

    signals[mode].if_updated(|value| self.set_mode(value));
    signals[freq].if_updated(|value| self.set_freq(value));
    signals[freq_mod].if_updated(|value| self.set_freq_mod(value));
    signals[q].if_updated(|value| self.set_q(value));

    let input = signals[self.block.input].get();
    let output = match self.mode {
      Mode::PassThrough => input,
      Mode::VAOnePole(_) => self.va_one_pole.process(input),
      Mode::OberheimSEM(_) => self.oberheim_sem.process(input),
    };

    signals[self.block.output].set(output);
  }
}

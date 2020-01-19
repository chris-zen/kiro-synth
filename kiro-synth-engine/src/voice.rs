use core::ops::DerefMut;
use heapless::Vec;

use crate::float::Float;
use crate::key_freqs::KEY_FREQ;
use crate::program::{MaxSignals, MaxBlocks, Block, osc, Program, SignalRef, ParamRef};
use crate::processor::Processor;
use crate::synth::SynthWaveforms;
use crate::signal::{Signal, SignalBus, SignalState};

pub(crate) struct Voice<'a, F: Float> {
  waveforms: &'a SynthWaveforms<F>,
  signals: Vec<Signal<F>, MaxSignals>,
  processors: Vec<Processor<'a, F>, MaxBlocks>,
}

impl<'a, F: Float> Voice<'a, F> {
  pub fn new(sample_rate: F, waveforms: &'a SynthWaveforms<F>, program: &Program<F>) -> Self {
    let mut signals: Vec<Signal<F>, MaxSignals> = Vec::new();
    for _ in 0..program.get_signals_count() {
      drop(signals.push(Signal::default()));
    }

    let mut processors: Vec<Processor<'a, F>, MaxBlocks> = Vec::new();
    for block in program.get_blocks().iter() {
      if let Block::Const { value, signal } = block {
        signals[signal.0].set(*value)
      }
      else {
        drop(processors.push(Processor::new(sample_rate, waveforms, block)));
      }
    }

//    println!("voice::signals {:?}", signals.iter_mut().map(|s| (s.consume(), s.state())).collect::<Vec<(F, SignalState), MaxSignals>>());

    Voice {
      waveforms,
      signals,
      processors,
    }
  }

  pub fn get_key(&self) -> u8 {
    self.signals[Program::<F>::NOTE_KEY_SIGNAL_REF].get().to_u8().unwrap()
  }

  pub fn get_velocity(&self) -> F {
    self.signals[Program::<F>::NOTE_VELOCITY_SIGNAL_REF].get()
  }

  pub fn reset(&mut self) {
    SignalBus::new(self.signals.deref_mut()).reset();
  }

  pub fn note_on(&mut self, key: u8, velocity: F) {
    self.reset();
    self.signals[Program::<F>::NOTE_KEY_SIGNAL_REF].set(F::from(key).unwrap());
    self.signals[Program::<F>::NOTE_VELOCITY_SIGNAL_REF].set(velocity);
    self.signals[Program::<F>::NOTE_PITCH_SIGNAL_REF].set(F::from(KEY_FREQ[(key & 0x7f) as usize]).unwrap());
  }

  pub fn note_off(&mut self) {
  }

  pub fn process(&mut self, program: &mut Program<F>) {
    let mut signals = SignalBus::new(self.signals.deref_mut());
    for processor in self.processors.iter_mut() {
      processor.process(&mut signals, program)
    }
    signals.update();
//    println!("{:?}", self.signals.iter_mut().map(|s| (s.get(), s.state())).collect::<Vec<(F, SignalState), MaxSignals>>());
  }

  pub fn output(&self) -> (F, F) {
    (
      self.signals[Program::<F>::OUTPUT_LEFT_SIGNAL_REF].get(),
      self.signals[Program::<F>::OUTPUT_RIGHT_SIGNAL_REF].get()
    )
  }
}

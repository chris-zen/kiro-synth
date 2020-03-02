use core::ops::DerefMut;
use heapless::Vec;

use crate::float::Float;
use crate::key_freqs::KEY_FREQ;
use crate::program::{MaxSignals, MaxBlocks, Block, osc, Program, SignalRef, ParamRef};
use crate::processor::Processor;
use crate::synth::{SynthWaveforms, SynthGlobals};
use crate::signal::{Signal, SignalBus, SignalState};

pub(crate) struct Voice<F: Float> {
  signals: Vec<Signal<F>, MaxSignals>,
  processors: Vec<Processor<F>, MaxBlocks>,
}

impl<F: Float> Voice<F> {
  pub fn new(sample_rate: F, program: &Program<F>) -> Self {
    let mut signals: Vec<Signal<F>, MaxSignals> = Vec::new();
    for _ in 0..program.get_signals_count() {
      drop(signals.push(Signal::default()));
    }

    let mut processors: Vec<Processor<F>, MaxBlocks> = Vec::new();
    for block in program.get_blocks().iter() {
      if let Block::Const { value, signal } = block {
        signals[signal.0].set(*value)
      }
      else {
        drop(processors.push(Processor::new(sample_rate, block)));
      }
    }

//    println!("voice::signals {:?}", signals.iter_mut().map(|s| (s.consume(), s.state())).collect::<Vec<(F, SignalState), MaxSignals>>());

    Voice {
      signals,
      processors,
    }
  }

  pub fn get_key(&self, program: &Program<F>) -> u8 {
    self.signals[program.voice().key.0].get().to_u8().unwrap()
  }
//
//  pub fn get_velocity(&self, program: &Program<F>) -> F {
//    self.signals[program.voice().velocity.0].get()
//  }

  pub fn is_off(&self, program: &Program<F>) -> bool {
    self.signals[program.voice().off.0].get() == F::one()
  }

  pub fn reset(&mut self, program: &Program<F>) {
    let mut signals = SignalBus::new(self.signals.deref_mut());
    signals.reset();

    for block in program.get_blocks() {
      if let Block::Param(param_block) = block {
        if let Some((_, param)) = program.get_param(param_block.reference) {
          let param_value = param.signal.get();
          signals[param_block.signal].set(param_value);
        }
      }
    }

    signals[program.voice().off].set(F::zero());

    for proc in self.processors.iter_mut() {
      proc.reset();
    }
  }

  pub fn note_on(&mut self, program: &Program<F>, key: u8, velocity: F) {
    self.reset(program);
    let voice = program.voice();
    self.signals[voice.key.0].set(F::val(key));
    self.signals[voice.velocity.0].set(velocity);
    self.signals[voice.note_pitch.0].set(F::val(KEY_FREQ[(key & 0x7f) as usize]));
    self.signals[voice.gate.0].set(F::one());
    self.signals[voice.trigger.0].set(F::one());
  }

  pub fn note_off(&mut self, program: &Program<F>) {
    self.signals[program.voice().gate.0].set(F::zero());
  }

  pub fn process(&mut self, program: &mut Program<F>, synth_globals: &SynthGlobals<F>) {
    let mut signals = SignalBus::new(self.signals.deref_mut());

    for processor in self.processors.iter_mut() {
      processor.process(&mut signals, program, synth_globals)
    }

    signals.update();

    // The trigger does an spike of 1 sample
    let voice = program.voice();
    if signals[voice.trigger].get() > F::zero() {
      signals[voice.trigger].set(F::zero())
    }

//    println!("{:?}", self.signals.iter_mut().skip(3).take(2).map(|s| (s.get(), s.state())).collect::<Vec<(F, SignalState), MaxSignals>>());
  }

  pub fn output(&self, program: &Program<F>) -> (F, F) {
    let voice = program.voice();
    (
      self.signals[voice.output_left.0].get(),
      self.signals[voice.output_right.0].get()
    )
  }
}

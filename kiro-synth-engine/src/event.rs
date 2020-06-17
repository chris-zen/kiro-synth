use crate::float::Float;
use crate::program::{ParamRef, SourceRef};

#[derive(Debug, Clone)]
pub enum Message<F: Float> {
  NoteOn {
    key: u8,
    velocity: F
  },
  NoteOff {
    key: u8,
    velocity: F
  },
  Param {
    param_ref: ParamRef,
    value: F,
  },
  ParamChange {
    param_ref: ParamRef,
    change: F,
  },
  ModulationAmount {
    source_ref: SourceRef,
    param_ref: ParamRef,
    amount: F,
  },
}

#[derive(Debug, Clone)]
pub struct Event<F: Float> {
  pub timestamp: u64,
  pub message: Message<F>,
}

impl<F: Float> Event<F> {
  pub fn new(timestamp: u64, message: Message<F>) -> Self {
    Event {
      timestamp,
      message,
    }
  }

  pub fn now(message: Message<F>) -> Self {
    Event {
      timestamp: 0,
      message,
    }
  }
}

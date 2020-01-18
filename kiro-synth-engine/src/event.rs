use num_traits::Float;

pub enum Message<F: Float> {
  NoteOn {
    key: u8,
    velocity: F
  },
  NoteOff {
    key: u8,
    velocity: F
  },

}

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

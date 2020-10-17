use crate::messages::Message;
use crate::types::{U14, U4, U7};

const NOTE_OFF_MASK: u8 = 0b1000_0000;
const NOTE_ON_MASK: u8 = 0b1001_0000;
const POLYPHONIC_KEY_PRESSURE_MASK: u8 = 0b1010_0000;
const CONTROL_CHANGE_MASK: u8 = 0b1011_0000;
const PROGRAM_CHANGE_MASK: u8 = 0b1100_0000;
const CHANNEL_PRESSURE_MASK: u8 = 0b1101_0000;
const PITCH_BEND_MASK: u8 = 0b1110_0000;
const SYSTEM_MASK: u8 = 0b1111_0000;

const SYSEX_START_CODE: u8 = 0b1111_0000;
const MTC_QUARTER_FRAME_CODE: u8 = 0b1111_0001;
const SONG_POSITION_POINTER_CODE: u8 = 0b1111_0010;
const SONG_SELECT_CODE: u8 = 0b1111_0011;
const TUNE_REQUEST_CODE: u8 = 0b1111_0110;
const SYSEX_END_CODE: u8 = 0b1111_0111;

const TIMING_CLOCK_CODE: u8 = 0b1111_1000;
const START_CODE: u8 = 0b1111_1010;
const CONTINUE_CODE: u8 = 0b1111_1011;
const STOP_CODE: u8 = 0b1111_1100;
const ACTIVE_SENSING_CODE: u8 = 0b1111_1110;
const SYSTEM_RESET_CODE: u8 = 0b1111_1111;

pub type Result<T> = core::result::Result<T, Error>;

fn is_midi_status(value: u8) -> bool {
  (value & 0x80) == 0x80
}

fn u14_from_u7_parts(lsb: U7, msb: U7) -> U14 {
  (U14::from(msb) << 7) | U14::from(lsb)
}

pub enum CallbackResult {
  Continue,
  Stop,
}

pub trait DecoderCallbacks: Sized {
  fn on_message(&mut self, message: Message) -> CallbackResult;
  fn on_sysex(&mut self, data: &[U7]) -> CallbackResult;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
  /// Missing more data while decoding
  MissingData,

  /// The status byte is not an expected message status
  UnexpectedStatus(u8),

  /// Found an unexpected value for a Channel Mode message.
  /// The error includes the controller number and the value that was found.
  UnexpectedChannelModeValue(U7, U7),

  /// The internal SysEx buffer does not have enough capacity to hold all the decoded data
  DataBufferOverflow,

  /// The decoder has been instructed to stop from a callback call
  Stopped,
}

#[derive(Debug, Clone, Copy)]
enum State {
  Start,
  Partial(u8, usize, usize),
  Message(Message),
  SysEx(usize),
  Finished,
}

impl State {
  pub fn finished(&self) -> bool {
    match self {
      State::Finished => true,
      _ => false,
    }
  }
}

pub struct Decoder<'a> {
  state: State,
  source_buffer: Option<u8>,
  data_buffer: &'a mut [U7],
}

impl<'a> Decoder<'a> {
  pub fn new(data_buffer: &'a mut [U7]) -> Decoder<'a> {
    Decoder {
      state: State::Start,
      source_buffer: None,
      data_buffer,
    }
  }

  pub fn decode<'b, Source, Callbacks>(
    &mut self,
    source: &mut Source,
    callbacks: &mut Callbacks,
  ) -> Result<()>
  where
    Source: Iterator<Item = &'b u8>,
    Callbacks: DecoderCallbacks,
  {
    while !self.state.finished() {
      self.decode_step(source, callbacks)?;
    }

    self.state = State::Start;

    Ok(())
  }

  fn decode_step<'b, Source, Callbacks>(
    &mut self,
    source: &mut Source,
    callbacks: &mut Callbacks,
  ) -> Result<()>
  where
    Source: Iterator<Item = &'b u8>,
    Callbacks: DecoderCallbacks,
  {
    let mut state = self.state;
    match &mut state {
      State::Start => self.decode_start(source)?,
      State::Partial(status, len, max_len) => {
        self.decode_partial(source, callbacks, *status, *len, *max_len)?
      }
      State::Message(message) => {
        self.state = match callbacks.on_message(*message) {
          CallbackResult::Continue => Ok(State::Start),
          CallbackResult::Stop => Err(Error::Stopped),
        }?;
      }
      State::SysEx(len) => self.decode_sysex(source, callbacks, *len)?,
      State::Finished => self.state = State::Start,
    };

    Ok(())
  }

  fn decode_start<'b, Source: Iterator<Item = &'b u8>>(
    &mut self,
    source: &mut Source,
  ) -> Result<()> {
    self.state = match self.read_next(source) {
      Ok(status) => Self::next_state_for_status(status),
      Err(Error::MissingData) => Ok(State::Finished),
      Err(err) => Err(err),
    }?;
    Ok(())
  }

  fn decode_partial<'b, Source, Callbacks>(
    &mut self,
    source: &mut Source,
    callbacks: &mut Callbacks,
    status: u8,
    mut len: usize,
    required: usize,
  ) -> Result<()>
  where
    Source: Iterator<Item = &'b u8>,
    Callbacks: DecoderCallbacks,
  {
    let limit = required.min(self.data_buffer.len());
    while len < limit && !self.source_is_empty(source) && !self.next_is_status(source) {
      self.data_buffer[len] = self.read_next(source)?;
      len += 1;
    }

    if len >= self.data_buffer.len() && len < required {
      self.state = State::Partial(status, len, required);
      Err(Error::DataBufferOverflow)
    } else if len < required {
      self.state = State::Partial(status, len, required);
      if self.source_is_empty(source) {
        Err(Error::MissingData)
      } else {
        self.decode_interleaved_realtime_messages(source, callbacks)
      }
    } else {
      match Self::decode_channel_message(status, self.data_buffer) {
        Ok(message) => {
          self.state = State::Message(message);
          Ok(())
        }
        Err(Error::UnexpectedStatus(_)) => {
          match Self::decode_system_common_message(status, self.data_buffer) {
            Ok(message) => {
              self.state = State::Message(message);
              Ok(())
            }
            Err(err) => {
              self.state = State::Start;
              Err(err)
            }
          }
        }
        Err(err) => {
          self.state = State::Start;
          Err(err)
        }
      }
    }
  }

  fn decode_sysex<'b, Source, Callbacks>(
    &mut self,
    source: &mut Source,
    callbacks: &mut Callbacks,
    mut len: usize,
  ) -> Result<()>
  where
    Source: Iterator<Item = &'b u8>,
    Callbacks: DecoderCallbacks,
  {
    while len < self.data_buffer.len()
      && !self.source_is_empty(source)
      && !self.next_is_status(source)
    {
      self.data_buffer[len] = self.read_next(source)?;
      len += 1;
    }

    if len >= self.data_buffer.len()
      && !self.source_is_empty(source)
      && !self.next_is_status(source)
    {
      self.state = State::SysEx(len);
      Err(Error::DataBufferOverflow)
    } else if self.source_is_empty(source) {
      self.state = State::SysEx(len);
      Err(Error::MissingData)
    } else {
      self.state = match self.decode_interleaved_realtime_messages(source, callbacks) {
        // all the real time messages have been processed, continue with the sysex data
        Ok(()) => Ok(State::SysEx(len)),

        // end of the sysex message
        Err(Error::UnexpectedStatus(status)) if status == SYSEX_END_CODE => {
          match callbacks.on_sysex(&self.data_buffer[0..len]) {
            CallbackResult::Continue => Ok(State::Start),
            CallbackResult::Stop => {
              self.state = State::Finished;
              Err(Error::Stopped)
            }
          }
        }

        // there was en error decoding the realtime messages
        Err(err) => Err(err),
      }?;
      Ok(())
    }
  }

  fn decode_interleaved_realtime_messages<'b, Source, Callbacks>(
    &mut self,
    source: &mut Source,
    callbacks: &mut Callbacks,
  ) -> Result<()>
  where
    Source: Iterator<Item = &'b u8>,
    Callbacks: DecoderCallbacks,
  {
    while self.next_is_status(source) {
      let value = self.read_next(source)?;
      let message = Self::decode_realtime_message(value)?;
      match callbacks.on_message(message) {
        CallbackResult::Continue => Ok(()),
        CallbackResult::Stop => Err(Error::Stopped),
      }?;
    }
    Ok(())
  }

  pub fn source_is_empty<'b, Source: Iterator<Item = &'b u8>>(
    &mut self,
    source: &mut Source,
  ) -> bool {
    self.peek_next(source).is_none()
  }

  pub fn next_is_status<'b, Source: Iterator<Item = &'b u8>>(
    &mut self,
    source: &mut Source,
  ) -> bool {
    self.peek_next(source).map(is_midi_status).unwrap_or(false)
  }

  pub fn read_next<'b, Source: Iterator<Item = &'b u8>>(
    &mut self,
    source: &mut Source,
  ) -> Result<u8> {
    let maybe_byte = self.peek_next(source);
    self.source_buffer = source.next().cloned();
    maybe_byte.ok_or(Error::MissingData)
  }

  fn peek_next<'b, Source: Iterator<Item = &'b u8>>(&mut self, source: &mut Source) -> Option<u8> {
    if self.source_buffer.is_none() {
      self.source_buffer = source.next().cloned();
    }
    self.source_buffer
  }

  fn next_state_for_status(status: u8) -> Result<State> {
    match status & 0xf0 {
      NOTE_OFF_MASK => Self::partial_state(status, 2),
      NOTE_ON_MASK => Self::partial_state(status, 2),
      POLYPHONIC_KEY_PRESSURE_MASK => Self::partial_state(status, 2),
      CONTROL_CHANGE_MASK => Self::partial_state(status, 2),
      PROGRAM_CHANGE_MASK => Self::partial_state(status, 1),
      CHANNEL_PRESSURE_MASK => Self::partial_state(status, 1),
      PITCH_BEND_MASK => Self::partial_state(status, 2),
      SYSTEM_MASK => Self::next_state_for_system_status(status),
      _ => Err(Error::UnexpectedStatus(status)),
    }
  }

  fn next_state_for_system_status(status: u8) -> Result<State> {
    match Self::decode_realtime_message(status) {
      Ok(message) => Ok(State::Message(message)),
      Err(Error::UnexpectedStatus(status)) => match status {
        SYSEX_START_CODE => Ok(State::SysEx(0)),
        MTC_QUARTER_FRAME_CODE => Self::partial_state(status, 1),
        SONG_POSITION_POINTER_CODE => Self::partial_state(status, 2),
        SONG_SELECT_CODE => Self::partial_state(status, 1),
        TUNE_REQUEST_CODE => Self::message_state(Message::TuneRequest),
        _ => Err(Error::UnexpectedStatus(status)),
      },
      Err(err) => Err(err),
    }
  }

  fn decode_realtime_message(status: u8) -> Result<Message> {
    match status {
      TIMING_CLOCK_CODE => Ok(Message::TimingClock),
      START_CODE => Ok(Message::Start),
      CONTINUE_CODE => Ok(Message::Continue),
      STOP_CODE => Ok(Message::Stop),
      ACTIVE_SENSING_CODE => Ok(Message::ActiveSensing),
      SYSTEM_RESET_CODE => Ok(Message::SystemReset),
      _ => Err(Error::UnexpectedStatus(status)),
    }
  }

  fn decode_channel_message(status: u8, data: &[U7]) -> Result<Message> {
    let channel = status & 0x0f;
    match status & 0xf0 {
      NOTE_OFF_MASK => Self::note_off(channel, data),
      NOTE_ON_MASK => Self::note_on(channel, data),
      POLYPHONIC_KEY_PRESSURE_MASK => Self::polyphonic_key_pressure(channel, data),
      CONTROL_CHANGE_MASK => Self::control_change(channel, data),
      PROGRAM_CHANGE_MASK => Self::program_change(channel, data),
      CHANNEL_PRESSURE_MASK => Self::channel_pressure(channel, data),
      PITCH_BEND_MASK => Self::pitch_bend(channel, data),
      _ => Err(Error::UnexpectedStatus(status)),
    }
  }

  // SysEx messages follow a different path through the internal state machine
  fn decode_system_common_message(status: u8, data: &[U7]) -> Result<Message> {
    match status {
      MTC_QUARTER_FRAME_CODE => Self::mtc_quarter_frame(data),
      SONG_POSITION_POINTER_CODE => Self::song_position_pointer(data),
      SONG_SELECT_CODE => Self::song_select(data),
      _ => Err(Error::UnexpectedStatus(status)),
    }
  }

  fn note_off(channel: U4, data: &[U7]) -> Result<Message> {
    Ok(Message::NoteOff {
      channel,
      key: data[0],
      velocity: data[1],
    })
  }

  fn note_on(channel: U4, data: &[U7]) -> Result<Message> {
    Ok(Message::NoteOn {
      channel,
      key: data[0],
      velocity: data[1],
    })
  }

  fn polyphonic_key_pressure(channel: U4, data: &[U7]) -> Result<Message> {
    Ok(Message::PolyphonicKeyPressure {
      channel,
      key: data[0],
      value: data[1],
    })
  }

  fn control_change(channel: U4, data: &[U7]) -> Result<Message> {
    let (controller, value) = (data[0], data[1]);
    match controller {
      120 => match value {
        0 => Ok(Message::AllSoundOff { channel }),
        _ => Err(Error::UnexpectedChannelModeValue(controller, value)),
      },
      121 => Ok(Message::ResetAllControllers { channel }),
      122 => match value {
        0 => Ok(Message::LocalControlOff { channel }),
        127 => Ok(Message::LocalControlOn { channel }),
        _ => Err(Error::UnexpectedChannelModeValue(controller, value)),
      },
      123 => match value {
        0 => Ok(Message::AllNotesOff { channel }),
        _ => Err(Error::UnexpectedChannelModeValue(controller, value)),
      },
      124 => match value {
        0 => Ok(Message::OmniModeOff { channel }),
        _ => Err(Error::UnexpectedChannelModeValue(controller, value)),
      },
      125 => match value {
        0 => Ok(Message::OmniModeOn { channel }),
        _ => Err(Error::UnexpectedChannelModeValue(controller, value)),
      },
      126 => Ok(Message::MonoModeOn {
        channel,
        num_channels: value,
      }),
      127 => match value {
        0 => Ok(Message::PolyModeOn { channel }),
        _ => Err(Error::UnexpectedChannelModeValue(controller, value)),
      },
      _ => Ok(Message::ControlChange {
        channel,
        controller,
        value,
      }),
    }
  }

  fn program_change(channel: U4, data: &[U7]) -> Result<Message> {
    Ok(Message::ProgramChange {
      channel,
      value: data[0],
    })
  }

  fn channel_pressure(channel: U4, data: &[U7]) -> Result<Message> {
    Ok(Message::ChannelPressure {
      channel,
      value: data[0],
    })
  }

  fn pitch_bend(channel: U4, data: &[U7]) -> Result<Message> {
    Ok(Message::PitchBend {
      channel,
      value: u14_from_u7_parts(data[0], data[1]),
    })
  }

  fn mtc_quarter_frame(data: &[U7]) -> Result<Message> {
    Ok(Message::MTCQuarterFrame {
      msg_type: (data[0] >> 4) & 0x07,
      value: data[0] & 0x0f,
    })
  }

  fn song_position_pointer(data: &[U7]) -> Result<Message> {
    Ok(Message::SongPositionPointer {
      beats: u14_from_u7_parts(data[0], data[1]),
    })
  }

  fn song_select(data: &[U7]) -> Result<Message> {
    Ok(Message::SongSelect { song: data[0] })
  }

  #[inline]
  fn partial_state(status: u8, remaining: usize) -> Result<State> {
    Ok(State::Partial(status, 0, remaining))
  }

  #[inline]
  fn message_state(message: Message) -> Result<State> {
    Ok(State::Message(message))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::decoder::Error::Stopped;
  use crate::messages::Message;
  use std::cell::{Ref, RefCell, RefMut};
  use std::rc::Rc;

  struct MockedCallbacksState {
    pub messages: Vec<Message>,
    pub sysex: Vec<U7>,
    pub num_events: usize,
    max_events: usize,
  }

  impl MockedCallbacksState {
    pub fn new() -> Self {
      MockedCallbacksState {
        messages: Vec::new(),
        sysex: Vec::new(),
        num_events: 0,
        max_events: usize::max_value(),
      }
    }
  }

  struct MockedCallbacks {
    state: Rc<RefCell<MockedCallbacksState>>,
  }

  impl MockedCallbacks {
    pub fn new(state: Rc<RefCell<MockedCallbacksState>>) -> Self {
      MockedCallbacks { state }
    }

    pub fn state(&self) -> Ref<MockedCallbacksState> {
      (*self.state).borrow()
    }

    pub fn state_mut(&self) -> RefMut<MockedCallbacksState> {
      (*self.state).borrow_mut()
    }

    fn event_result(&self, mut state: RefMut<MockedCallbacksState>) -> CallbackResult {
      state.num_events += 1;
      if state.num_events >= state.max_events {
        CallbackResult::Stop
      } else {
        CallbackResult::Continue
      }
    }
  }

  impl DecoderCallbacks for MockedCallbacks {
    fn on_message(&mut self, message: Message) -> CallbackResult {
      println!("on_message({:?})", message);
      let mut state = (*self.state).borrow_mut();
      state.messages.push(message);
      self.event_result(state)
    }

    fn on_sysex(&mut self, data: &[u8]) -> CallbackResult {
      println!("on_sysex({:?})", data);
      let mut state = (*self.state).borrow_mut();
      state.sysex.extend_from_slice(data);
      self.event_result(state)
    }
  }

  fn decoder_with_mocked_callbacks<F>(sysex_buffer: &mut [U7], mut f: F)
  where
    F: FnMut(MockedCallbacks, Decoder),
  {
    let callbacks_state = Rc::new(RefCell::new(MockedCallbacksState::new()));
    let callbacks = MockedCallbacks::new(callbacks_state);
    let decoder = Decoder::new(sysex_buffer);
    (f)(callbacks, decoder)
  }

  fn decodes_successfully(source: Vec<u8>, expected: Vec<Message>) {
    let mut source = source.iter();
    decoder_with_mocked_callbacks(&mut [0u8; 2], |mut callbacks, mut decoder| {
      assert_eq!(decoder.decode(&mut source, &mut callbacks), Ok(()));
      assert_eq!(callbacks.state().messages, expected);
    });
  }

  #[test]
  fn decode_empty_vec() {
    decoder_with_mocked_callbacks(&mut [], |mut callbacks, mut decoder| {
      let data = Vec::<u8>::new();
      let mut source = data.iter();
      assert_eq!(decoder.decode(&mut source, &mut callbacks), Ok(()));
      assert_eq!(callbacks.state().num_events, 0);
    });
  }

  #[test]
  fn reserved_status() {
    decoder_with_mocked_callbacks(&mut [0u8; 2], |mut callbacks, mut decoder| {
      let data = vec![0b1111_0100, 0b1111_0101, 0b1111_1001, 0b1111_1101];
      let mut source = data.iter();

      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_0100))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_0101))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_1001))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_1101))
      );
    });
  }

  #[test]
  fn unexpected_status() {
    decoder_with_mocked_callbacks(&mut [0u8; 2], |mut callbacks, mut decoder| {
      let data = vec![0b1000_0000, 64, 0b1000_0001, 0b1000_0010, 12];
      let mut source = data.iter();

      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1000_0001))
      );
      assert_eq!(callbacks.state().num_events, 0);
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1000_0010))
      );
      assert_eq!(callbacks.state().num_events, 0);
      assert_eq!(decoder.decode(&mut source, &mut callbacks), Ok(()));
      assert_eq!(
        callbacks.state().messages,
        vec![Message::NoteOff {
          channel: 0,
          key: 64,
          velocity: 12
        }]
      );
    });
  }

  #[test]
  fn decode_notes() {
    decodes_successfully(
      vec![0b1000_0101u8, 64, 127, 0b1001_1010, 0, 127],
      vec![
        Message::NoteOff {
          channel: 0b0101,
          key: 64,
          velocity: 127,
        },
        Message::NoteOn {
          channel: 0b1010,
          key: 0,
          velocity: 127,
        },
      ],
    );
  }

  #[test]
  fn decode_polyphonic_key_pressure() {
    decodes_successfully(
      vec![0b1010_0101u8, 64, 127],
      vec![Message::PolyphonicKeyPressure {
        channel: 0b0101,
        key: 64,
        value: 127,
      }],
    );
  }

  #[test]
  fn decode_control_change() {
    decodes_successfully(
      vec![0b1011_0101u8, 64, 127],
      vec![Message::ControlChange {
        channel: 0b0101,
        controller: 64,
        value: 127,
      }],
    );
  }

  #[test]
  fn unexpected_channel_mode() {
    decoder_with_mocked_callbacks(&mut [0u8; 2], |mut callbacks, mut decoder| {
      let data = vec![
        0b1011_0101u8,
        120,
        1,
        0b1011_0101u8,
        122,
        1,
        0b1011_0101u8,
        123,
        1,
        0b1011_0101u8,
        124,
        1,
        0b1011_0101u8,
        125,
        1,
        0b1011_0101u8,
        127,
        1,
      ];
      let mut source = data.iter();

      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedChannelModeValue(120, 1))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedChannelModeValue(122, 1))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedChannelModeValue(123, 1))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedChannelModeValue(124, 1))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedChannelModeValue(125, 1))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedChannelModeValue(127, 1))
      );
    });
  }

  #[test]
  fn decode_program_change() {
    decodes_successfully(
      vec![0b1100_0101u8, 0b0_1010101],
      vec![Message::ProgramChange {
        channel: 0b0101,
        value: 0b0_1010101,
      }],
    );
  }

  #[test]
  fn decode_channel_pressure() {
    decodes_successfully(
      vec![0b1101_0101u8, 0b0_1010101],
      vec![Message::ChannelPressure {
        channel: 0b0101,
        value: 0b0_1010101,
      }],
    );
  }

  #[test]
  fn decode_pitch_bend() {
    decodes_successfully(
      vec![0b1110_0101u8, 0b0_1010101, 0b0_0101010],
      vec![Message::PitchBend {
        channel: 0b0101,
        value: 0b0_01010101010101,
      }],
    );
  }

  #[test]
  #[allow(clippy::inconsistent_digit_grouping)]
  fn decode_mtc_quarter_frame() {
    decodes_successfully(
      vec![0b1111_0001u8, 0b0_101_1010],
      vec![Message::MTCQuarterFrame {
        msg_type: 0b101,
        value: 0b1010,
      }],
    );
  }

  #[test]
  fn decode_song_position_pointer() {
    decodes_successfully(
      vec![
        0b1111_0010u8,
        0b0_1010101,
        0b0_0101010,
        0b1111_0010u8,
        0b0_0101010,
        0b0_1010101,
      ],
      vec![
        Message::SongPositionPointer {
          beats: 0b01_0101_0101_0101,
        },
        Message::SongPositionPointer {
          beats: 0b10_1010_1010_1010,
        },
      ],
    );
  }

  #[test]
  fn decode_song_select() {
    decodes_successfully(
      vec![0b1111_0011u8, 0b0_1010101],
      vec![Message::SongSelect { song: 0b101_0101 }],
    );
  }

  #[test]
  fn decode_tune_request() {
    decodes_successfully(vec![0b1111_0110u8], vec![Message::TuneRequest]);
  }

  #[test]
  fn decode_timing_clock() {
    decodes_successfully(vec![0b1111_1000u8], vec![Message::TimingClock]);
  }

  #[test]
  fn decode_start_continue_stop() {
    decodes_successfully(
      vec![0b1111_1010u8, 0b1111_1011, 0b1111_1100],
      vec![Message::Start, Message::Continue, Message::Stop],
    );
  }

  #[test]
  fn decode_active_sensing() {
    decodes_successfully(vec![0b1111_1110u8], vec![Message::ActiveSensing]);
  }

  #[test]
  fn decode_system_reset() {
    decodes_successfully(vec![0b1111_1111u8], vec![Message::SystemReset]);
  }

  #[test]
  fn decode_reserved() {
    decoder_with_mocked_callbacks(&mut [], |mut callbacks, mut decoder| {
      let data = vec![0b1111_0100u8, 0b1111_0101, 0b1111_1001, 0b1111_1101];
      let mut source = data.iter();

      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_0100))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_0101))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_1001))
      );
      assert_eq!(
        decoder.decode(&mut source, &mut callbacks),
        Err(Error::UnexpectedStatus(0b1111_1101))
      );
      assert_eq!(callbacks.state().num_events, 0);
    });
  }

  #[test]
  fn decode_interleaved_real_time() {
    decodes_successfully(
      vec![0b1000_0101u8, 0b1111_1000u8, 64, 0b1111_1010u8, 127],
      vec![
        Message::TimingClock,
        Message::Start,
        Message::NoteOff {
          channel: 0b0101,
          key: 64,
          velocity: 127,
        },
      ],
    );
  }

  #[test]
  fn sysex_buffer_overflow() {
    decoder_with_mocked_callbacks(&mut [0u8; 2], |mut callbacks, mut decoder| {
      let data = vec![0b1111_0000u8, 1, 2, 3, 4, 0b1111_0111];
      assert_eq!(
        decoder.decode(&mut data.iter(), &mut callbacks),
        Err(Error::DataBufferOverflow)
      );
    });
  }

  #[test]
  fn decode_sysex_continuous() {
    decoder_with_mocked_callbacks(&mut [0u8; 4], |mut callbacks, mut decoder| {
      let data = vec![0b1111_0000u8, 1, 2, 3, 4, 0b1111_0111];

      assert_eq!(decoder.decode(&mut data.iter(), &mut callbacks), Ok(()));
      assert_eq!(callbacks.state().sysex, vec![1u8, 2, 3, 4]);
    });
  }

  #[test]
  fn decode_sysex_interleaved() {
    decoder_with_mocked_callbacks(&mut [0u8; 4], |mut callbacks, mut decoder| {
      let data = vec![
        0b1111_0000u8,
        0b1111_1000u8,
        1,
        2,
        0b1111_1010u8,
        0b1111_1011,
        3,
        4,
        0b1111_1100,
        0b1111_0111,
      ];

      assert_eq!(decoder.decode(&mut data.iter(), &mut callbacks), Ok(()));
      assert_eq!(
        callbacks.state().messages,
        vec![
          Message::TimingClock,
          Message::Start,
          Message::Continue,
          Message::Stop
        ]
      );
      assert_eq!(callbacks.state().sysex, vec![1u8, 2, 3, 4]);
    });
  }

  #[test]
  fn decode_sysex_interleaved_split() {
    decoder_with_mocked_callbacks(&mut [0u8; 4], |mut callbacks, mut decoder| {
      let dataset = vec![
        0b1111_0000u8,
        0b1111_1000u8,
        1,
        2,
        0b1111_1010u8,
        0b1111_1011,
        3,
        4,
        0b1111_1100,
      ]
      .into_iter()
      .map(|value| vec![value])
      .collect::<Vec<Vec<u8>>>();

      dataset.iter().take(9).for_each(|data| {
        assert_eq!(
          decoder.decode(&mut data.iter(), &mut callbacks),
          Err(Error::MissingData)
        );
      });

      let source = vec![0b1111_0111];
      assert_eq!(decoder.decode(&mut source.iter(), &mut callbacks), Ok(()));

      assert_eq!(
        callbacks.state().messages,
        vec![
          Message::TimingClock,
          Message::Start,
          Message::Continue,
          Message::Stop
        ]
      );
      assert_eq!(callbacks.state().sysex, vec![1u8, 2, 3, 4]);
    });
  }

  #[test]
  fn decode_stop() {
    decoder_with_mocked_callbacks(&mut [0u8; 2], |mut callbacks, mut decoder| {
      let data = vec![0b1000_0000, 64, 0b1111_1000u8, 12, 0b1111_1010u8];
      let mut source = data.iter();
      callbacks.state_mut().max_events = 1;

      assert_eq!(decoder.decode(&mut source, &mut callbacks), Err(Stopped));
      assert_eq!(callbacks.state().messages, vec![Message::TimingClock]);

      callbacks.state_mut().max_events = 100;

      assert_eq!(decoder.decode(&mut source, &mut callbacks), Ok(()));
      assert_eq!(
        callbacks.state().messages,
        vec![
          Message::TimingClock,
          Message::NoteOff {
            channel: 0,
            key: 64,
            velocity: 12
          },
          Message::Start,
        ]
      );
    });
  }
}

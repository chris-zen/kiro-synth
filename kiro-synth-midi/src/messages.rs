use crate::types::{U14, U3, U4, U7};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Message {
  // --- Channel Voice Messages [nnnn = 0-15 (MIDI Channel Number 1-16)]
  /// This message is sent when a note is released (ended).
  NoteOff { channel: U4, key: U7, velocity: U7 },

  /// This message is sent when a note is depressed (start).
  NoteOn { channel: U4, key: U7, velocity: U7 },

  /// This message is most often sent by pressing down on the key after it "bottoms out".
  PolyphonicKeyPressure { channel: U4, key: U7, value: U7 },

  /// This message is sent when a controller value changes.
  ControlChange {
    channel: U4,
    controller: U7,
    value: U7,
  },

  /// This message sent when the patch number changes.
  ProgramChange { channel: U4, value: U7 },

  /// This message is most often sent by pressing down on the key after it "bottoms out".
  /// Use this message to send the single greatest pressure value (of all the current depressed keys).
  ChannelPressure { channel: U4, value: U7 },

  /// Pitch Bend Change. This message is sent to indicate a change in the pitch bender
  /// (wheel or lever, typically). The pitch bender is measured by a fourteen bit value. Center
  /// (no pitch change) is 2000H.
  PitchBend { channel: U4, value: U14 },

  // --- Channel Mode Messages
  /// When All Sound Off is received all oscillators will turn off, and their
  /// volume envelopes are set to zero as soon as possible.
  AllSoundOff { channel: U4 },

  /// When Reset All Controllers is received, all controller values are reset to their default values.
  ResetAllControllers { channel: U4 },

  /// When Local Control is Off, all devices on a given channel will respond only to data
  /// received over MIDI. Played data, etc. will be ignored.
  LocalControlOff { channel: U4 },

  /// Local Control On restores the functions of the normal controllers.
  LocalControlOn { channel: U4 },

  /// When an All Notes Off is received, all oscillators will turn off.
  AllNotesOff { channel: U4 },

  /// It also causes all notes off.
  OmniModeOff { channel: U4 },

  /// It also causes all notes off.
  OmniModeOn { channel: U4 },

  /// It also causes all notes off.
  MonoModeOn { channel: U4, num_channels: U7 },

  /// It also causes all notes off.
  PolyModeOn { channel: U4 },

  // --- System Common Messages
  /// MIDI Time Code Quarter Frame.
  /// The type determines how to interpret the value:
  /// 0 Current Frames Low Nibble
  /// 1 Current Frames High Nibble
  /// 2 Current Seconds Low Nibble
  /// 3 Current Seconds High Nibble
  /// 4 Current Minutes Low Nibble
  /// 5 Current Minutes High Nibble
  /// 6 Current Hours Low Nibble
  /// 7 Current Hours High Nibble and SMPTE Type
  MTCQuarterFrame { msg_type: U3, value: U4 },

  /// Song Position Pointer.
  /// This is an internal 14 bit register that holds the number of MIDI beats
  /// (1 beat= six MIDI clocks) since the start of the song.
  SongPositionPointer { beats: U14 },

  /// Song Select
  /// The Song Select specifies which sequence or song is to be played.
  SongSelect { song: U7 },

  /// Upon receiving a Tune Request, all analog synthesizers should tune their oscillators.
  TuneRequest,

  /// System Exclusive Messages
  SysEx { length: u16 },

  // --- System Real-Time Messages
  /// Timing Clock. Sent 24 times per quarter note when synchronization is required.
  TimingClock,

  /// Start the current sequence playing.
  /// (This message will be followed with Timing Clocks).
  Start,

  /// Continue at the point the sequence was Stopped
  Continue,

  /// Stop the current sequence.
  Stop,

  /// This message is intended to be sent repeatedly to tell the receiver that a
  /// connection is alive. Use of this message is optional. When initially received, the receiver
  /// will expect to receive another Active Sensing message each 300ms (max), and if it does not
  /// then it will assume that the connection has been terminated. At termination, the receiver
  /// will turn off all voices and return to normal (non- active sensing) operation.
  ActiveSensing,

  /// Reset all receivers in the system to power-up status. This should be used sparingly,
  /// preferably under manual control. In particular, it should not be sent on power-up.
  SystemReset,
}

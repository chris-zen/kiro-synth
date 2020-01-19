mod audio;
mod midi;
mod programs;

use anyhow::Result;

use ringbuf::{Consumer, Producer, RingBuffer};

use kiro_synth_core::waveforms::saw;
use kiro_synth_core::waveforms::saw::Saw;
use kiro_synth_core::waveforms::Waveform;
use kiro_synth_core::float::Float;

use kiro_synth_midi::messages::Message as MidiMessage;

use kiro_synth_engine::synth::{Synth, SynthWaveforms};
use kiro_synth_engine::event::{Event, Message as SynthMessage};

use crate::audio::{AudioDriver, AudioHandler};
use crate::midi::{MidiDriver, MidiHandler};
use crate::programs::Programs;

const SAMPLE_RATE: u32 = 44100;

const MIDI_BUFFER_SIZE: usize = 512;
static mut MIDI_BUFFER: [u8; MIDI_BUFFER_SIZE] = [0; MIDI_BUFFER_SIZE];

fn main() -> Result<()> {
  // CONFIG

  let midi_buffer: &'static mut [u8] = unsafe { MIDI_BUFFER.as_mut() };

  let waveforms = SynthWaveforms::new();

  // EVENTS

  let events_ring_buffer = RingBuffer::<Event<f32>>::new(1024);
  let (events_producer, events_consumer) = events_ring_buffer.split();

  // SYNTH

  let synth = Synth::new(SAMPLE_RATE as f32, events_consumer, &waveforms, Programs::default());

  // MIDI

  let handler = EventsMidiHandler::new(events_producer);
  let midi_driver = MidiDriver::new("kiro-synth", midi_buffer, handler)?;

  // AUDIO

  let audio_driver = AudioDriver::new(SAMPLE_RATE)?;
  audio_driver.run(SynthAudioHandler(synth));

  drop(midi_driver);

  Ok(())
}

struct SynthAudioHandler<'a>(Synth<'a, f32>);

impl<'a> AudioHandler for SynthAudioHandler<'a> {
  fn prepare(&mut self, len: usize) {
    self.0.prepare();
  }

  fn next(&mut self) -> (f32, f32) {
    self.0.process()
  }
}

struct EventsMidiHandler {
  events: Producer<Event<f32>>
}

impl EventsMidiHandler {
  pub fn new(events: Producer<Event<f32>>) -> Self {
    EventsMidiHandler { events }
  }
}

impl MidiHandler for EventsMidiHandler {
  fn on_message(&mut self, timestamp: u64, message: MidiMessage) {
    println!("{:014}: {:?}", timestamp, message);
    match message {
      MidiMessage::NoteOn { channel: _, key, velocity } => {
        drop(self.events.push(Event::new(0u64, SynthMessage::NoteOn { key, velocity: velocity as f32 / 128.0 })));
      },
      MidiMessage::NoteOff { channel: _, key, velocity } => {
        drop(self.events.push(Event::new(0u64, SynthMessage::NoteOff { key, velocity: velocity as f32 / 128.0 })));
      },
      _ => {}
    };
  }

  fn on_sysex(&mut self, timestamp: u64, data: &[u8]) {
    println!("{:014}: {:?}", timestamp, data);
    // TODO unimplemented!()
  }
}

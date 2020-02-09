mod audio;
mod midi;
mod programs;
mod midi_mapper;

use anyhow::Result;

use ringbuf::{Producer, RingBuffer};

use kiro_synth_midi::messages::Message as MidiMessage;

use kiro_synth_engine::synth::{Synth, SynthWaveforms};
use kiro_synth_engine::event::{Event, Message as SynthMessage};

use crate::audio::{AudioDriver, AudioHandler};
use crate::midi::{MidiDriver, MidiHandler};
use crate::programs::PlaygroundModule;
use crate::midi_mapper::MidiMapper;

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

  let (program, module) = PlaygroundModule::new_program(waveforms.len());

  let mut midi_mapper = MidiMapper::new();
  midi_mapper.pitch_bend(program.get_param(module.params.pitch_bend));

  midi_mapper.rel_controller(21, program.get_param(module.params.osc1_amplitude));
  midi_mapper.rel_controller(22, program.get_param(module.params.osc1_shape));
  midi_mapper.rel_controller(23, program.get_param(module.params.osc1_octave));
  midi_mapper.rel_controller(24, program.get_param(module.params.osc1_semitones));
  midi_mapper.rel_controller(25, program.get_param(module.params.osc1_cents));

  midi_mapper.rel_controller(26, program.get_param(module.params.dca_amplitude));
  midi_mapper.rel_controller(27, program.get_param(module.params.dca_pan));

//  midi_mapper.rel_controller(29, program.get_param(module.params.osc2_amplitude));
//  midi_mapper.rel_controller(30, program.get_param(module.params.osc2_shape));
//  midi_mapper.rel_controller(31, program.get_param(module.params.osc2_octave));
//  midi_mapper.rel_controller(32, program.get_param(module.params.osc2_semitones));
//  midi_mapper.rel_controller(33, program.get_param(module.params.osc2_cents));

  midi_mapper.rel_controller(29, program.get_param(module.params.eg1_attack));
  midi_mapper.rel_controller(30, program.get_param(module.params.eg1_decay));
  midi_mapper.rel_controller(31, program.get_param(module.params.eg1_sustain));
  midi_mapper.rel_controller(32, program.get_param(module.params.eg1_release));
  midi_mapper.rel_controller(33, program.get_param(module.params.eg1_mode));
  midi_mapper.rel_controller(34, program.get_param(module.params.eg1_legato));
  midi_mapper.rel_controller(35, program.get_param(module.params.eg1_reset_to_zero));
  midi_mapper.rel_controller(36, program.get_param(module.params.eg1_dca_intensity));

  midi_mapper.rel_controller(41, program.get_param(module.params.osc3_amplitude));
  midi_mapper.rel_controller(42, program.get_param(module.params.osc3_shape));
  midi_mapper.rel_controller(43, program.get_param(module.params.osc3_octave));
  midi_mapper.rel_controller(44, program.get_param(module.params.osc3_semitones));
  midi_mapper.rel_controller(45, program.get_param(module.params.osc3_cents));

  midi_mapper.rel_controller(49, program.get_param(module.params.osc4_amplitude));
  midi_mapper.rel_controller(50, program.get_param(module.params.osc4_shape));
  midi_mapper.rel_controller(51, program.get_param(module.params.osc4_octave));
  midi_mapper.rel_controller(52, program.get_param(module.params.osc4_semitones));
  midi_mapper.rel_controller(53, program.get_param(module.params.osc4_cents));

  midi_mapper.rel_controller(54, program.get_param(module.params.filt1_mode));
  midi_mapper.rel_controller(55, program.get_param(module.params.filt1_freq));
  midi_mapper.rel_controller(56, program.get_param(module.params.filt1_q));

  let synth = Synth::new(SAMPLE_RATE as f32, events_consumer, &waveforms, program);

  // MIDI

  let handler = EventsMidiHandler::new(midi_mapper, events_producer);
  let midi_driver = MidiDriver::new("kiro-synth", midi_buffer, handler)?;

  // AUDIO

  let audio_driver = AudioDriver::new(SAMPLE_RATE)?;
  audio_driver.run(SynthAudioHandler(synth));

  drop(midi_driver);

  Ok(())
}

struct SynthAudioHandler<'a>(Synth<'a, f32>);

impl<'a> AudioHandler for SynthAudioHandler<'a> {
  fn prepare(&mut self, _len: usize) {
    self.0.prepare();
  }

  fn next(&mut self) -> (f32, f32) {
    self.0.process()
  }
}

struct EventsMidiHandler {
  midi_mapper: MidiMapper<f32>,
  events: Producer<Event<f32>>,
}

impl EventsMidiHandler {
  pub fn new(midi_mapper: MidiMapper<f32>, events: Producer<Event<f32>>) -> Self {
    EventsMidiHandler {
      midi_mapper,
      events
    }
  }
}

impl MidiHandler for EventsMidiHandler {
  fn on_message(&mut self, timestamp: u64, message: MidiMessage) {
    println!("{:014}: {:?}", timestamp, message);
    match message {
      MidiMessage::NoteOn { channel: _, key, velocity } => {
        let message = SynthMessage::NoteOn { key, velocity: velocity as f32 / 127.0 };
        drop(self.events.push(Event::new(0u64, message)));
      },
      MidiMessage::NoteOff { channel: _, key, velocity } => {
        let message = SynthMessage::NoteOff { key, velocity: velocity as f32 / 127.0 };
        drop(self.events.push(Event::new(0u64, message)));
      },
      MidiMessage::PitchBend { channel: _, value } => {
        if let Some(event) = self.midi_mapper.map_midi_pitch_bend(value) {
          drop(self.events.push(event));
        }
      },
      MidiMessage::ControlChange { channel: _, controller, value } => {
        if let Some(event) = self.midi_mapper.map_midi_controller(controller, value) {
          drop(self.events.push(event));
        }
      }
      _ => {}
    };
  }

  fn on_sysex(&mut self, timestamp: u64, data: &[u8]) {
    println!("{:014}: {:?}", timestamp, data);
    // TODO unimplemented!()
  }
}

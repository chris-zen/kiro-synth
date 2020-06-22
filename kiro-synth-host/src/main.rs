mod audio;
mod midi;
mod program;
mod synth;
pub mod ui;

use std::sync::{Mutex, Arc};

use anyhow::Result;
use ringbuf::RingBuffer;

use kiro_synth_core::float::Float;
use kiro_synth_midi::messages::Message as MidiMessage;
use kiro_synth_engine::program::Program;
use kiro_synth_engine::event::Event;
use kiro_synth_engine::synth::Synth;
use kiro_synth_engine::globals::SynthGlobals;

use crate::audio::{AudioDriver, AudioHandler};
use crate::midi::drivers::{MidiDriver, MidiHandler};
use crate::program::kiro::KiroModule;
use crate::midi::mapper::MidiMapper;
use crate::ui::{Synth as SynthData};
use crate::synth::{SynthClient, SynthClientMutex};

const SAMPLE_RATE: u32 = 44100;

const MIDI_BUFFER_SIZE: usize = 512;
static mut MIDI_BUFFER: [u8; MIDI_BUFFER_SIZE] = [0; MIDI_BUFFER_SIZE];

fn main() -> Result<()> {
  // CONFIG

  let midi_buffer: &'static mut [u8] = unsafe { MIDI_BUFFER.as_mut() };

  let synth_globals = SynthGlobals::new();

  // EVENTS

  let events_ring_buffer = RingBuffer::<Event<f32>>::new(1024);
  let (events_producer, events_consumer) = events_ring_buffer.split();
  let synth_client = Arc::new(Mutex::new(SynthClient::new(synth_globals.clone(), events_producer)));

  // PROGRAM

  let (program, module) = KiroModule::new_program(
    synth_globals.lfo_waveforms.len(),
    synth_globals.osc_waveforms.len());

  // UI DATA

  let synth_data = SynthData::new(&program, &module, SynthClientMutex::new(synth_client.clone()));

  // MIDI

  let midi_mapper = create_midi_mapper(&program, &module);
  let handler = EventsMidiHandler::new(midi_mapper, synth_client.clone());
  let _midi_driver = MidiDriver::new("kiro-synth", midi_buffer, handler)?;

  // SYNTH

  let synth = Synth::new(SAMPLE_RATE as f32, events_consumer, program, synth_globals);

  // AUDIO

  let handler = SynthAudioHandler(synth);
  let _audio_driver = AudioDriver::new(SAMPLE_RATE, handler)?;

  // UI

  ui::start(synth_data, synth_client);

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
  synth_client: Arc<Mutex<SynthClient<f32>>>,
}

impl EventsMidiHandler {
  pub fn new(midi_mapper: MidiMapper<f32>, synth_client: Arc<Mutex<SynthClient<f32>>>) -> Self {
    EventsMidiHandler {
      midi_mapper,
      synth_client
    }
  }
}

impl MidiHandler for EventsMidiHandler {
  fn on_message(&mut self, timestamp: u64, message: MidiMessage) {
    println!("{:014}: {:?}", timestamp, message);
    match message {
      MidiMessage::NoteOn { channel: _, key, velocity } => {
        self.synth_client.lock().unwrap().send_note_on(key, velocity as f32 / 127.0);
      },
      MidiMessage::NoteOff { channel: _, key, velocity } => {
        self.synth_client.lock().unwrap().send_note_off(key, velocity as f32 / 127.0);
      },
      MidiMessage::PitchBend { channel: _, value } => {
        if let Some(event) = self.midi_mapper.map_midi_pitch_bend(value) {
          self.synth_client.lock().unwrap().send_event(event);
        }
      },
      MidiMessage::ControlChange { channel: _, controller, value } => {
        if let Some(event) = self.midi_mapper.map_midi_controller(controller, value) {
          self.synth_client.lock().unwrap().send_event(event);
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

fn create_midi_mapper<F: Float>(program: &Program<F>, module: &KiroModule) -> MidiMapper<F> {
  let mut midi_mapper = MidiMapper::new();

  midi_mapper.pitch_bend(program.get_param(module.params.pitch_bend.reference));

  // midi_mapper.rel_controller(21, program.get_param(module.params.osc1.amplitude.reference));
  // midi_mapper.rel_controller(22, program.get_param(module.params.osc1.shape.reference));
  // midi_mapper.rel_controller(23, program.get_param(module.params.osc1.octave.reference));
  // midi_mapper.rel_controller(24, program.get_param(module.params.osc1.semitones.reference));
  // midi_mapper.rel_controller(25, program.get_param(module.params.osc1.cents.reference));

  midi_mapper.rel_controller(26, program.get_param(module.params.dca.amplitude.reference));
  midi_mapper.rel_controller(27, program.get_param(module.params.dca.pan.reference));

//  midi_mapper.rel_controller(29, program.get_param(module.params.osc2.amplitude.reference));
//  midi_mapper.rel_controller(30, program.get_param(module.params.osc2.shape.reference));
//  midi_mapper.rel_controller(31, program.get_param(module.params.osc2.octave.reference));
//  midi_mapper.rel_controller(32, program.get_param(module.params.osc2.semitones.reference));
//  midi_mapper.rel_controller(33, program.get_param(module.params.osc2.cents.reference));

  midi_mapper.rel_controller(29, program.get_param(module.params.eg1.attack.reference));
  midi_mapper.rel_controller(30, program.get_param(module.params.eg1.decay.reference));
  midi_mapper.rel_controller(31, program.get_param(module.params.eg1.sustain.reference));
  midi_mapper.rel_controller(32, program.get_param(module.params.eg1.release.reference));
  midi_mapper.rel_controller(33, program.get_param(module.params.eg1.mode.reference));
  midi_mapper.rel_controller(34, program.get_param(module.params.eg1.legato.reference));
  midi_mapper.rel_controller(35, program.get_param(module.params.eg1.reset_to_zero.reference));
  midi_mapper.rel_controller(36, program.get_param(module.params.eg1.dca_mod.reference));

  midi_mapper.rel_controller(41, program.get_param(module.params.osc3.amplitude.reference));
  midi_mapper.rel_controller(42, program.get_param(module.params.osc3.shape.reference));
  midi_mapper.rel_controller(43, program.get_param(module.params.osc3.octaves.reference));
  midi_mapper.rel_controller(44, program.get_param(module.params.osc3.semitones.reference));
  midi_mapper.rel_controller(45, program.get_param(module.params.osc3.cents.reference));

  midi_mapper.rel_controller(49, program.get_param(module.params.osc4.amplitude.reference));
  midi_mapper.rel_controller(50, program.get_param(module.params.osc4.shape.reference));
  midi_mapper.rel_controller(51, program.get_param(module.params.osc4.octaves.reference));
  midi_mapper.rel_controller(52, program.get_param(module.params.osc4.semitones.reference));
  midi_mapper.rel_controller(53, program.get_param(module.params.osc4.cents.reference));

  midi_mapper.rel_controller(54, program.get_param(module.params.filter1.mode.reference));
  midi_mapper.rel_controller(55, program.get_param(module.params.filter1.freq.reference));
  midi_mapper.rel_controller(56, program.get_param(module.params.filter1.q.reference));

  midi_mapper
}

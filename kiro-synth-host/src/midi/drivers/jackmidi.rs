use anyhow::Result;
use jack::{Client, Control, ProcessHandler, ProcessScope};
use thiserror::Error;

use kiro_midi_core::decoder::{CallbackResult, Decoder, DecoderCallbacks};
use kiro_midi_core::messages::Message;

use crate::midi::drivers::MidiHandler;

struct MidiDecoderCallbacks<'a, Handler: MidiHandler> {
  timestamp: u64,
  handler: &'a mut Handler,
}

impl<'a, Handler> DecoderCallbacks for MidiDecoderCallbacks<'a, Handler>
where
  Handler: MidiHandler,
{
  fn on_message(&mut self, message: Message) -> CallbackResult {
    self.handler.on_message(self.timestamp, message);
    CallbackResult::Continue
  }

  fn on_sysex(&mut self, data: &[u8]) -> CallbackResult {
    self.handler.on_sysex(self.timestamp, data);
    CallbackResult::Continue
  }
}

#[derive(Error, Debug)]
#[error("Jack returned ClientStatus: {0:?}")]
pub struct ClientStatusError(jack::ClientStatus);

#[derive(Error, Debug)]
#[error("Error registering port: {0}")]
pub struct PortRegisterError(String);

#[derive(Error, Debug)]
pub enum JackMidiError {
  #[error("Error creating a new client")]
  ClientCreate(#[source] ClientStatusError),

  #[error("Error creating an input port")]
  PortCreate(#[source] PortRegisterError),
}

struct JackProcessHandler<'a, Handler: MidiHandler> {
  input_port: jack::Port<jack::MidiIn>,
  decoder: Decoder<'a>,
  handler: Handler,
}

impl<'a, Handler: MidiHandler> ProcessHandler for JackProcessHandler<'a, Handler> {
  fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
    for raw_midi in self.input_port.iter(ps) {
      let timestamp = raw_midi.time as u64;
      let mut callbacks = MidiDecoderCallbacks {
        timestamp,
        handler: &mut self.handler,
      };
      let mut source = raw_midi.bytes.iter();
      self.decoder.decode(&mut source, &mut callbacks).ok();
    }
    jack::Control::Continue
  }
}

pub struct JackMidiDriver<'a, Handler: MidiHandler> {
  _client: jack::AsyncClient<(), JackProcessHandler<'a, Handler>>,
}

impl<'a, Handler> JackMidiDriver<'a, Handler>
where
  Handler: MidiHandler + Send + 'static,
{
  pub fn new(app_name: &str, midi_buffer: &'static mut [u8], handler: Handler) -> Result<Self> {
    let (client, _status) = jack::Client::new(app_name, jack::ClientOptions::NO_START_SERVER)
      .map_err(|error| {
        let status = match error {
          jack::Error::ClientError(status) => status,
          _ => jack::ClientStatus::empty(),
        };
        JackMidiError::ClientCreate(ClientStatusError(status))
      })?;
    let input_port = client
      .register_port("midi_in", jack::MidiIn)
      .map_err(|error| {
        let port_name = match error {
          jack::Error::PortRegistrationError(port_name) => port_name,
          _ => "Unknown".to_string(),
        };
        JackMidiError::PortCreate(PortRegisterError(port_name))
      })?;
    let decoder = Decoder::new(midi_buffer);
    let midi_handler = JackProcessHandler {
      input_port,
      decoder,
      handler,
    };
    let _client = client.activate_async((), midi_handler)?;
    Ok(Self { _client })
  }
}

use anyhow::Result;
use thiserror::Error;

use coremidi::{Client, InputPort, PacketList, Sources};

use kiro_midi_core::decoder::{CallbackResult, Decoder, DecoderCallbacks};
use kiro_midi_core::messages::Message;

use crate::midi::drivers::{MidiError, MidiHandler};

#[derive(Error, Debug)]
#[error("CoreMidi returned OSStatus: {0}")]
pub struct OSStatusError(i32);

#[derive(Error, Debug)]
pub enum CoreMidiError {
  #[error("Error creating a new client")]
  ClientCreate(#[source] OSStatusError),

  #[error("Error creating an input port")]
  PortCreate(#[source] OSStatusError),
}

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

pub struct CoreMidiDriver {
  _client: Client,
  _input_port: InputPort,
}

impl CoreMidiDriver {
  pub fn new<Handler: MidiHandler + Send + 'static>(
    app_name: &str,
    midi_buffer: &'static mut [u8],
    mut handler: Handler,
  ) -> Result<Self> {
    let client = Client::new(app_name).map_err(|status| {
      MidiError::DriverInit(CoreMidiError::ClientCreate(OSStatusError(status)))
    })?;

    println!("CoreMidi client created");

    let mut decoder = Decoder::new(midi_buffer);

    let input_port = client
      .input_port(app_name, move |packet_list: &PacketList| {
        Self::port_callback(packet_list, &mut decoder, &mut handler)
      })
      .map_err(|status| MidiError::DriverInit(CoreMidiError::PortCreate(OSStatusError(status))))?;

    Sources.into_iter().for_each(|source| {
      let display_name = source
        .display_name()
        .unwrap_or_else(|| "Unknown source".to_string());
      println!("Connecting source '{}'", display_name);
      match input_port.connect_source(&source) {
        Ok(_) => (),
        Err(err) => eprintln!("Error connecting source '{}': {}", display_name, err),
      }
    });

    println!("CoreMidi input port created");

    Ok(Self {
      _client: client,
      _input_port: input_port,
    })
  }

  fn port_callback<Handler: MidiHandler + 'static>(
    packet_list: &PacketList,
    decoder: &mut Decoder,
    handler: &mut Handler,
  ) {
    //    println!("{:?}", packet_list);
    for packet in packet_list.iter() {
      let timestamp = packet.timestamp();
      let mut source = packet.data().iter();
      let mut callbacks = MidiDecoderCallbacks { timestamp, handler };
      decoder.decode(&mut source, &mut callbacks).ok();
    }
  }
}

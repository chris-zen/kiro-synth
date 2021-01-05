mod analog_osc;
mod audio_handler;
mod controller;
mod graph;

use crate::controller::controller_main;
use anyhow::Result;

fn main() -> Result<()> {
  controller_main()
}

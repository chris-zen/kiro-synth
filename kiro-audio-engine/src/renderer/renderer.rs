use std::ops::DerefMut;

use ringbuf::{Consumer, Producer};

use crate::messages::Message;
use crate::renderer::plan::RenderOp;
use crate::renderer::plan::RenderPlan;
use crate::EngineConfig;

pub struct Renderer {
  tx: Producer<Message>,
  rx: Consumer<Message>,

  plan: Box<RenderPlan>,
}

unsafe impl Send for Renderer {}

impl Renderer {
  pub fn new(tx: Producer<Message>, rx: Consumer<Message>, _config: EngineConfig) -> Self {
    let plan = Box::new(RenderPlan::default());

    Self { tx, rx, plan }
  }

  pub fn render(
    &mut self,
    input: &[f32],
    input_channels: usize,
    output: &mut [f32],
    output_channels: usize,
  ) {
    self.process_messages();
    self.render_plan(input, input_channels, output, output_channels);
  }

  fn process_messages(&mut self) {
    while let Some(message) = self.rx.pop() {
      match message {
        Message::MoveRenderPlan(plan) => {
          let prev_plan = std::mem::replace(&mut self.plan, plan);
          self.tx.push(Message::MoveRenderPlan(prev_plan)).ok(); // FIXME this will deallocate if failure
        }
      }
    }
  }

  fn render_plan(
    &mut self,
    _input: &[f32],
    _input_channels: usize,
    output: &mut [f32],
    output_channels: usize,
  ) {
    for op in self.plan.operations.iter_mut() {
      match op {
        RenderOp::RenderOutput(port) => {
          let num_channels = port.len().min(output_channels);
          for channel_index in 0..num_channels {
            let buffer = port.channel(channel_index);
            let mut output_offset = channel_index;
            for sample in buffer.as_slice() {
              output[output_offset] = *sample;
              output_offset += output_channels;
            }
          }
          for _channel_index in num_channels..output_channels {
            //TODO fill with zeros
          }
        }
        RenderOp::RenderProcessor(processor_ref, context) => {
          let processor = processor_ref.deref_mut();
          processor.render(context);
        }
      }
    }
  }
}

use std::ops::DerefMut;

use ringbuf::{Consumer, Producer};

use crate::messages::Message;
use crate::processor::context::RenderContext;
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
    let num_samples = output.len() / output_channels;
    output.iter_mut().for_each(|s| *s = 0.0);
    // TODO Check if the mut can be removed
    for op in self.plan.operations.iter_mut() {
      match op {
        RenderOp::RenderOutput {
          alias: _,
          audio_input,
        } => {
          for (channel_index, input_buffer) in audio_input.iter().enumerate() {
            let mut output_offset = channel_index;
            for sample in input_buffer.as_slice()[0..num_samples].iter() {
              output[output_offset] = *sample;
              output_offset += output_channels;
            }
          }
        }
        RenderOp::RenderProcessor {
          processor_ref,
          audio_inputs,
          audio_outputs,
          parameters,
        } => {
          audio_inputs
            .values_mut()
            .for_each(|port| port.set_num_samples(num_samples));
          audio_outputs
            .values_mut()
            .for_each(|port| port.set_num_samples(num_samples));
          parameters
            .values_mut()
            .for_each(|port| port.set_num_samples(num_samples));

          let mut context =
            RenderContext::new(num_samples, audio_inputs, audio_outputs, parameters);

          let processor = processor_ref.deref_mut();
          // TODO Check if the mut can be removed
          processor.render(&mut context);
        }
      }
    }
  }
}

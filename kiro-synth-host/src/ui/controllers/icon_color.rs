use druid::{Widget, Selector, Env, Data, Color, EventCtx, Event};
use druid::widget::Controller;
use druid_icon::Icon;

pub struct IconColorController<T, ID> {
  selector: Selector<(ID, Color)>,
  id: Box<dyn Fn(&T) -> ID + 'static>
}

impl<T: Data, ID: PartialEq> IconColorController<T, ID> {
  pub fn new(selector: Selector<(ID, Color)>, id: impl Fn(&T) -> ID + 'static) -> Self {
    IconColorController {
      selector,
      id: Box::new(id),
    }
  }
}

impl<T: Data, ID: PartialEq + 'static> Controller<T, Icon<T>> for IconColorController<T, ID> {
  fn event(&mut self, child: &mut Icon<T>, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    match event {
      Event::Command(command) if command.is(self.selector) => {
        if let Some((target_id, color)) = command.get(self.selector) {
          if *target_id == (self.id)(data) {
            child.set_color(color.clone())
          }
        }
      }
      _ => child.event(ctx, event, data, env),
    }
  }
}

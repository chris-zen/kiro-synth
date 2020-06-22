use druid::{Widget, Selector, Env, Data, LifeCycleCtx, LifeCycle, Command};
use druid::widget::Controller;

pub struct HotChangedController<T, ID, V> {
  selector: Selector<(ID, V)>,
  id: Box<dyn Fn(&T) -> ID + 'static>,
  value: Box<dyn Fn(bool, &T, &Env) -> V + 'static>,
}

impl<T: Data, ID: PartialEq + 'static, V: Clone + 'static> HotChangedController<T, ID, V> {
  pub fn new(selector: Selector<(ID, V)>,
             id: impl Fn(&T) -> ID + 'static,
             value: impl Fn(bool, &T, &Env) -> V + 'static) -> Self {
    
    HotChangedController {
      selector,
      id: Box::new(id),
      value: Box::new(value),
    }
  }
}

impl<T: Data, W: Widget<T>, ID: PartialEq + 'static, V: Clone + 'static> Controller<T, W> for HotChangedController<T, ID, V> {
  fn lifecycle(
    &mut self,
    child: &mut W,
    ctx: &mut LifeCycleCtx,
    event: &LifeCycle,
    data: &T,
    env: &Env,
  ) {
    if let LifeCycle::HotChanged(is_hot) = event {
      let id: ID = (self.id)(data);
      let value: V = (self.value)(*is_hot, data, env);
      let command = Command::new(self.selector, (id, value));
      ctx.submit_command(command, None);
    }
    else {
      child.lifecycle(ctx, event, data, env);
    }
  }
}

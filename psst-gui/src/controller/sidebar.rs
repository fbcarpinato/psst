use crate::{cmd::TOGGLE_SIDEBAR, data::AppState};
use druid::Widget;

pub struct SidebarController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for SidebarController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::Command(cmd) if cmd.is(TOGGLE_SIDEBAR) => {
                data.toggle_sidebar();
                ctx.request_layout();
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

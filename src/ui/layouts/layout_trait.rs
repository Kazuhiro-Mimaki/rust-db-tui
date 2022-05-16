use tui::{backend::Backend, layout::Rect, Frame};

use crate::ui::widgets::ctx::WidgetCtx;

pub trait LayoutTrait {
    fn new(size: Rect) -> Self;
    fn render_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx);
}

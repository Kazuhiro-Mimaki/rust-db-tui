use tui::{backend::Backend, layout::Rect, Frame};

use crate::ui::widgets::ctx::WidgetCtx;

pub trait LayoutTrait {
    fn new(size: Rect) -> Self;
    fn render_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx);
}

pub trait NormalLayoutTrait {
    fn new(size: Rect) -> Self;
    fn render_record_table_layout<B: Backend>(
        &self,
        f: &mut Frame<'_, B>,
        widget_ctx: &mut WidgetCtx,
    );
    fn render_column_table_layout<B: Backend>(
        &self,
        f: &mut Frame<'_, B>,
        widget_ctx: &mut WidgetCtx,
    );
    fn render_base_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx);
}

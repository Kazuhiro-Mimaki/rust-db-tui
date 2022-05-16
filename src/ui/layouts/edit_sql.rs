use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::ui::widgets::ctx::WidgetCtx;
use unicode_width::UnicodeWidthStr;

use super::layout_trait::LayoutTrait;

pub struct EditSQLLayout {
    left_side_widget: Vec<Rect>,
    main_widget: Vec<Rect>,
}

impl LayoutTrait for EditSQLLayout {
    fn new(size: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(15), Constraint::Length(85)].as_ref())
            .split(size);

        let chunks_1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(chunks[0]);

        let chunks_2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(chunks[1]);

        Self {
            left_side_widget: chunks_1,
            main_widget: chunks_2,
        }
    }

    fn render_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx) {
        f.render_widget(
            widget_ctx.database.current_database_widget(),
            self.left_side_widget[0],
        );

        f.render_stateful_widget(
            widget_ctx.table_list.widget(),
            self.left_side_widget[1],
            &mut widget_ctx.table_list.table_select_state,
        );

        f.render_widget(widget_ctx.sql_input.widget(), self.main_widget[0]);
        f.set_cursor(
            // Put cursor past the end of the input text
            self.main_widget[0].x + widget_ctx.sql_input.input.width() as u16 + 1,
            // Move one line down, from the border to the input line
            self.main_widget[0].y + 1,
        );

        f.render_widget(widget_ctx.sql_output.widget(), self.main_widget[1]);
    }
}

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::ui::widgets::ctx::WidgetCtx;
use unicode_width::UnicodeWidthStr;

pub struct EditSQLLayout {
    left: Vec<Rect>,
    right: Vec<Rect>,
}

impl EditSQLLayout {
    pub fn new(size: Rect) -> Self {
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
            left: chunks_1,
            right: chunks_2,
        }
    }

    pub fn render_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx) {
        f.render_widget(widget_ctx.database.current_database_widget(), self.left[0]);

        f.render_stateful_widget(
            widget_ctx.table_list.widget(),
            self.left[1],
            &mut widget_ctx.table_list.table_select_state,
        );

        f.render_widget(widget_ctx.sql_input.widget(), self.right[0]);
        f.set_cursor(
            // Put cursor past the end of the input text
            self.right[0].x + widget_ctx.sql_input.input.width() as u16 + 1,
            // Move one line down, from the border to the input line
            self.right[0].y + 1,
        );

        f.render_widget(widget_ctx.sql_output.widget(), self.right[1]);
    }
}

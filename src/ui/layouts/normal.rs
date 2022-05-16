use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::ui::widgets::{ctx::WidgetCtx, tab::TableMode};

pub struct NormalLayout {
    left: Vec<Rect>,
    right: Vec<Rect>,
}

impl NormalLayout {
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
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
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

        f.render_widget(widget_ctx.tab.widget(), self.right[1]);

        match widget_ctx.tab.mode {
            TableMode::Records => {
                f.render_stateful_widget(
                    widget_ctx.table_record.widget(),
                    self.right[2],
                    &mut widget_ctx.table_record.select_row_list_state,
                );
            }
            TableMode::Columns => {
                f.render_stateful_widget(
                    widget_ctx.table_column.widget(),
                    self.right[2],
                    &mut widget_ctx.table_column.select_row_list_state,
                );
            }
        };
    }
}

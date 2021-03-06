use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::ui::widgets::{ctx::WidgetCtx, tab::TableMode};

use super::layout_trait::LayoutTrait;

pub struct ChangeDBLayout {
    left_side_widget: Rect,
    main_widget: Vec<Rect>,
}

impl LayoutTrait for ChangeDBLayout {
    fn new(size: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(15), Constraint::Length(85)].as_ref())
            .split(size);

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
            left_side_widget: chunks[0],
            main_widget: chunks_2,
        }
    }

    fn render_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx) {
        f.render_stateful_widget(
            widget_ctx.database.expand_db_list_widget(),
            self.left_side_widget,
            &mut widget_ctx.database.database_select_state,
        );

        f.render_widget(widget_ctx.sql_input.widget(), self.main_widget[0]);

        f.render_widget(widget_ctx.tab.widget(), self.main_widget[1]);

        match widget_ctx.tab.mode {
            TableMode::Records => {
                f.render_stateful_widget(
                    widget_ctx.table.record_widget.widget(),
                    self.main_widget[2],
                    &mut widget_ctx.table.record_widget.select_row_list_state,
                );
            }
            TableMode::Columns => {
                f.render_stateful_widget(
                    widget_ctx.table.column_widget.widget(),
                    self.main_widget[2],
                    &mut widget_ctx.table.column_widget.select_row_list_state,
                );
            }
        };
    }
}

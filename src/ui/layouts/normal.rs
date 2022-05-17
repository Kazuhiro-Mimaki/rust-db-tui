use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::ui::widgets::ctx::WidgetCtx;

use super::layout_trait::NormalLayoutTrait;

pub struct NormalLayout {
    left_side_widget: Vec<Rect>,
    main_widget: Vec<Rect>,
}

impl NormalLayoutTrait for NormalLayout {
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
            left_side_widget: chunks_1,
            main_widget: chunks_2,
        }
    }

    fn render_record_table_layout<B: Backend>(
        &self,
        f: &mut Frame<'_, B>,
        widget_ctx: &mut WidgetCtx,
    ) {
        self.render_base_layout(f, widget_ctx);

        f.render_stateful_widget(
            widget_ctx.table.record_widget.widget(),
            self.main_widget[2],
            &mut widget_ctx.table.record_widget.select_row_list_state,
        );
    }

    fn render_column_table_layout<B: Backend>(
        &self,
        f: &mut Frame<'_, B>,
        widget_ctx: &mut WidgetCtx,
    ) {
        self.render_base_layout(f, widget_ctx);

        f.render_stateful_widget(
            widget_ctx.table.column_widget.widget(),
            self.main_widget[2],
            &mut widget_ctx.table.column_widget.select_row_list_state,
        );
    }

    fn render_base_layout<B: Backend>(&self, f: &mut Frame<'_, B>, widget_ctx: &mut WidgetCtx) {
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

        f.render_widget(widget_ctx.tab.widget(), self.main_widget[1]);
    }
}

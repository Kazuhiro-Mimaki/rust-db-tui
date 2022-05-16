use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct TableListWdg<'a> {
    title: &'a str,
    pub tables: Vec<String>,
    pub table_select_state: ListState,
    pub current_table: String,
}

impl<'a> TableListWdg<'a> {
    pub fn new(tables: Vec<String>) -> Self {
        let mut table_select_state = ListState::default();
        table_select_state.select(Some(0));
        let current_table = tables[0].clone();

        Self {
            title: "Tables",
            tables: tables,
            table_select_state: table_select_state,
            current_table: current_table,
        }
    }

    pub fn widget(&self) -> List<'a> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);
        let table_names: Vec<_> = self
            .tables
            .clone()
            .into_iter()
            .map(|table_name| {
                ListItem::new(Spans::from(vec![Span::styled(
                    table_name,
                    Style::default(),
                )]))
            })
            .collect();
        let widget = List::new(table_names).block(block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
        return widget;
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.table_select_state.selected() {
            if !self.is_first(selected) {
                self.table_select_state.select(Some(selected - 1));
            };
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.table_select_state.selected() {
            if self.is_last(selected) {
                self.table_select_state.select(Some(selected + 1));
            }
        }
    }

    fn is_first(&self, selected: usize) -> bool {
        return selected == 0;
    }

    fn is_last(&self, selected: usize) -> bool {
        return selected < self.tables.len().saturating_sub(1);
    }

    pub fn change_table(&mut self) {
        if let Some(selected) = self.table_select_state.selected() {
            self.current_table = self.tables[selected].clone();
        }
    }

    pub fn change_tables(&mut self, new_tables: Vec<String>) {
        self.tables = new_tables;
    }
}

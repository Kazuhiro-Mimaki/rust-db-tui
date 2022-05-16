use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub struct DatabaseWdg<'a> {
    title: &'a str,
    databases: Vec<String>,
    pub current_database: String,
    pub database_select_state: ListState,
}

impl<'a> DatabaseWdg<'a> {
    pub fn new(databases: Vec<String>) -> Self {
        let mut database_select_state = ListState::default();
        database_select_state.select(Some(0));
        let current_database = databases[0].clone();

        Self {
            title: "DB",
            databases: databases,
            current_database: current_database,
            database_select_state: database_select_state,
        }
    }

    pub fn current_database_widget(&self) -> Paragraph<'a> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);
        let widget = Paragraph::new(self.current_database.to_string())
            .style(Style::default())
            .block(block);
        return widget;
    }

    pub fn expand_db_list_widget(&self) -> List<'a> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);
        let database_names: Vec<_> = self
            .databases
            .clone()
            .into_iter()
            .map(|db| ListItem::new(Spans::from(vec![Span::styled(db, Style::default())])))
            .collect();
        let widget = List::new(database_names).block(block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        return widget;
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.database_select_state.selected() {
            if !self.is_first(selected) {
                self.database_select_state.select(Some(selected - 1));
            };
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.database_select_state.selected() {
            if self.is_last(selected) {
                self.database_select_state.select(Some(selected + 1));
            }
        }
    }

    fn is_first(&self, selected: usize) -> bool {
        return selected == 0;
    }

    fn is_last(&self, selected: usize) -> bool {
        return selected < self.databases.len().saturating_sub(1);
    }

    pub fn change_database(&mut self) {
        if let Some(selected) = self.database_select_state.selected() {
            self.current_database = self.databases[selected].clone();
        }
    }
}

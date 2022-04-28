use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, ListState, Row as WdgRow, Table},
    Frame, Terminal,
};

mod db;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut table_list_state = ListState::default();
    table_list_state.select(Some(0));

    // Set config
    dotenv().ok();

    let mysql_client = db::MySqlClient::new(&env::var("DATABASE_URL").unwrap()).await;

    let table_rows = mysql_client
        .get_table_list(&env::var("DB_NAME").unwrap())
        .await;
    let tables = db::parse_sql_tables(table_rows);
    let default_table_name = &tables[0];

    let record_rows = mysql_client.get_record_list(default_table_name).await;
    let (headers, records) = db::parse_sql_records(record_rows);
    let mut table_struct = db::TableStruct::new(default_table_name.to_string(), headers, records);

    loop {
        terminal.draw(|f| render_layout(f, &tables, &mut table_struct, &mut table_list_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('a') => {
                    if let Some(selected) = table_list_state.selected() {
                        if selected != 0 {
                            table_list_state.select(Some(selected - 1));
                        };
                    }
                }
                KeyCode::Char('b') => {
                    if let Some(selected) = table_list_state.selected() {
                        table_list_state.select(Some(selected + 1));
                    }
                }
                KeyCode::Up => {
                    table_struct.move_up();
                }
                KeyCode::Down => {
                    table_struct.move_down();
                }
                KeyCode::Right => {
                    table_struct.move_right();
                }
                KeyCode::Left => {
                    table_struct.move_left();
                }
                KeyCode::Enter => {
                    if let Some(selected) = table_list_state.selected() {
                        let selected_table_name = &tables[selected];
                        if selected_table_name.to_string() != table_struct.name {
                            let record_rows =
                                mysql_client.get_record_list(selected_table_name).await;
                            let (headers, records) = db::parse_sql_records(record_rows);
                            table_struct
                                .reset_default_records(
                                    selected_table_name.to_string(),
                                    headers,
                                    records,
                                )
                                .await;
                        };
                    }
                }
                _ => {}
            }
        }
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn render_layout<B: Backend>(
    f: &mut Frame<'_, B>,
    tables: &Vec<String>,
    table_struct: &mut db::TableStruct,
    table_list_state: &mut ListState,
) {
    let size = f.size();

    let block_1 = Block::default().title("Block 1").borders(Borders::ALL);
    let block_2_1 = Block::default().title("Tables").borders(Borders::ALL);
    let block_2_2 = Block::default().title("Records").borders(Borders::ALL);

    let table_names: Vec<_> = tables
        .iter()
        .map(|table_name| {
            ListItem::new(Spans::from(vec![Span::styled(
                table_name,
                Style::default(),
            )]))
        })
        .collect();

    let table_list = List::new(table_names).block(block_2_1).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    table_struct.update_visible_range();

    let header_cells = table_struct.headers[table_struct.visible_start_column_index..]
        .iter()
        .enumerate()
        .map(|(_, h)| {
            Cell::from(h.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
        });
    let header_layout = WdgRow::new(header_cells).height(1).bottom_margin(1);

    let record_layout = table_struct
        .records
        .iter()
        .enumerate()
        .map(|(row_index, item)| {
            // let height = item
            //     .iter()
            //     .map(|content| content.chars().filter(|c| *c == '\n').count())
            //     .max()
            //     .unwrap_or(0)
            //     + 1;
            let cells = item[table_struct.visible_start_column_index..]
                .iter()
                .enumerate()
                .map(|(column_idx, c)| {
                    // このcolumn_idxに入るのは0~9
                    Cell::from(c.to_string()).style(
                        if column_idx
                            == table_struct.selected_column_index
                                - table_struct.visible_start_column_index
                            && Some(row_index) == table_struct.row_list_state.selected()
                        {
                            Style::default().bg(Color::Blue)
                        } else {
                            Style::default()
                        },
                    )
                });
            WdgRow::new(cells).bottom_margin(1)
        });

    let record_list = Table::new(record_layout)
        .header(header_layout)
        .block(block_2_2)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ]);

    let chunks_1 = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(size);

    let chunks_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Length(85)].as_ref())
        .split(chunks_1[1]);

    f.render_widget(block_1, chunks_1[0]);
    f.render_stateful_widget(table_list, chunks_2[0], table_list_state);
    f.render_stateful_widget(record_list, chunks_2[1], &mut table_struct.row_list_state);
}

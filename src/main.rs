use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use sqlx::{mysql::MySqlRow, Row};
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, Row as WdgRow, Table},
    Frame, Terminal,
};

mod db;
mod utils;

struct App {
    col_section_start_idx: usize,
    col_section_end_idx: usize,
}

impl App {
    fn set_col_start_idx(&mut self, new_col_section_start_idx: usize) {
        self.col_section_start_idx = new_col_section_start_idx;
    }

    fn set_col_end_idx(&mut self, new_col_section_end_idx: usize) {
        self.col_section_end_idx = new_col_section_end_idx;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        col_section_start_idx: 0,
        col_section_end_idx: 9,
    };

    // Set config
    dotenv().ok();

    let table_name = &env::var("TABLE_NAME").unwrap();

    let mysql_client = db::MySqlClient::new(&env::var("DATABASE_URL").unwrap()).await;

    let table_list = mysql_client
        .get_table_list(&env::var("DB_NAME").unwrap())
        .await;

    let record_rows = mysql_client.get_record_list(table_name).await;

    let (headers, records) = db::parse_sql_records(record_rows);
    let mut table_struct = db::TableStruct::new(table_name.to_string(), headers, records);

    loop {
        terminal.draw(|f| render_layout(f, &mut app, &table_list, &mut table_struct))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Up => {
                    table_struct.scroll_up();
                }
                KeyCode::Down => {
                    table_struct.scroll_down();
                }
                KeyCode::Right => {
                    table_struct.scroll_right();
                }
                KeyCode::Left => {
                    table_struct.scroll_left();
                }
                KeyCode::Enter => {
                    break;
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
    app: &mut App,
    table_list: &Vec<MySqlRow>,
    table_struct: &mut db::TableStruct,
) {
    let size = f.size();

    let block_1 = Block::default().title("Block 1").borders(Borders::ALL);
    let block_2_1 = Block::default().title("Tables").borders(Borders::ALL);
    let block_2_2 = Block::default().title("Records").borders(Borders::ALL);

    let tables: Vec<_> = table_list
        .iter()
        .map(|table| {
            ListItem::new(Spans::from(vec![Span::styled(
                table.get::<String, _>("Name"),
                Style::default(),
            )]))
        })
        .collect();

    let table_list = List::new(tables).block(block_2_1).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    if table_struct.selected_column_index > app.col_section_end_idx {
        app.set_col_end_idx(table_struct.selected_column_index);
        app.set_col_start_idx(app.col_section_end_idx - 9);
    } else if table_struct.selected_column_index < app.col_section_start_idx {
        app.set_col_start_idx(table_struct.selected_column_index);
        app.set_col_end_idx(app.col_section_start_idx + 9);
    }

    let header_cells = table_struct.headers[app.col_section_start_idx..]
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
            let cells =
                item[app.col_section_start_idx..]
                    .iter()
                    .enumerate()
                    .map(|(column_idx, c)| {
                        // このcolumn_idxに入るのは0~9
                        Cell::from(c.to_string()).style(
                            if column_idx
                                == table_struct.selected_column_index - app.col_section_start_idx
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
    f.render_widget(table_list, chunks_2[0]);
    f.render_stateful_widget(record_list, chunks_2[1], &mut table_struct.row_list_state);
}

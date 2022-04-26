use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use sqlx::{
    mysql::{MySqlRow},
    Column, MySqlPool, Row,
};
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, Row as WdgRow, Table, TableState},
    Frame, Terminal,
};

struct App {
    col_section_start_idx: usize,
    col_section_end_idx: usize,
    selected_column_index: usize,
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

    let mut table_list_state = TableState::default();
    table_list_state.select(Some(0));

    let mut mut_headers = vec![];

    let mut app = App {
        col_section_start_idx: 0,
        col_section_end_idx: 9,
        selected_column_index: 0,
    };

    // Set config
    dotenv().ok();

    // ====================
    // Set database
    // ====================
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // ====================
    // Show table name list
    // ====================
    let get_tables_query = format!(
        "{} {}",
        "SHOW TABLE STATUS FROM",
        &env::var("DB_NAME").unwrap()
    );
    let table_rows = sqlx::query(&get_tables_query.as_str())
        .fetch_all(&pool)
        .await
        .unwrap();

    // ====================
    // Show records for the table
    // ====================
    let get_records = format!("{} {}", "SELECT * FROM", &env::var("TABLE_NAME").unwrap());
    let record_rows = sqlx::query(&get_records.as_str())
        .fetch_all(&pool)
        .await
        .unwrap();

    loop {
        terminal.draw(|f| {
            render_layout(
                f,
                &mut table_list_state,
                &mut mut_headers,
                &mut app,
                &table_rows,
                &record_rows,
            )
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Down => {
                    if let Some(selected) = table_list_state.selected() {
                        table_list_state.select(Some(selected + 1));
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = table_list_state.selected() {
                        if selected != 0 {
                            table_list_state.select(Some(selected - 1));
                        };
                    }
                }
                KeyCode::Right => {
                    if app.selected_column_index < mut_headers.len() - 1 {
                        app.selected_column_index += 1;
                    }
                }
                KeyCode::Left => {
                    if app.selected_column_index != 0 {
                        app.selected_column_index -= 1;
                    }
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
    table_list_state: &mut TableState,
    mut_headers: &mut Vec<String>,
    app: &mut App,
    table_rows: &Vec<MySqlRow>,
    record_rows: &Vec<MySqlRow>,
) {
    let size = f.size();

    let block_1 = Block::default().title("Block 1").borders(Borders::ALL);
    let block_2_1 = Block::default().title("Tables").borders(Borders::ALL);
    let block_2_2 = Block::default().title("Records").borders(Borders::ALL);

    let tables: Vec<_> = table_rows
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

    let mut records = vec![];

    for row in record_rows.iter() {
        *mut_headers = row
            .columns()
            .iter()
            .map(|column| column.name().to_string())
            .collect();

        let mut new_row = vec![];
        for column in row.columns() {
            let column_name = column.name();
            new_row.push(convert_column_value_to_string(&row, column_name));
        }
        records.push(new_row);
    }

    if app.selected_column_index > app.col_section_end_idx {
        app.set_col_end_idx(app.selected_column_index);
        app.set_col_start_idx(app.col_section_end_idx - 9);
    } else if app.selected_column_index < app.col_section_start_idx {
        app.set_col_start_idx(app.selected_column_index);
        app.set_col_end_idx(app.col_section_start_idx + 9);
    }

    let header_cells = mut_headers[app.col_section_start_idx..]
        .iter()
        .enumerate()
        .map(|(column_index, h)| {
            Cell::from(h.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
        });
    let header_layout = WdgRow::new(header_cells).height(1).bottom_margin(1);

    let record_layout = records.iter().enumerate().map(|(row_index, item)| {
        // let height = item
        //     .iter()
        //     .map(|content| content.chars().filter(|c| *c == '\n').count())
        //     .max()
        //     .unwrap_or(0)
        //     + 1;
        let cells = item[app.col_section_start_idx..]
            .iter()
            .enumerate()
            .map(|(column_idx, c)| {
                // このcolumn_idxに入るのは0~9
                Cell::from(c.to_string()).style(Style::default().bg(
                    if column_idx == app.selected_column_index - app.col_section_start_idx
                        && Some(row_index) == table_list_state.selected()
                    {
                        Color::Blue
                    } else {
                        Color::Reset
                    },
                ))
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
    f.render_stateful_widget(record_list, chunks_2[1], table_list_state);
}

fn convert_column_value_to_string(row: &MySqlRow, column_name: &str) -> String {
    if let Ok(value) = row.try_get(column_name) {
        let value: String = value;
        value
    } else {
        String::from("null")
    }
}

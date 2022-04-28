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
    widgets::ListState,
    Frame, Terminal,
};

mod db;
mod table;
mod ui;
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

    let mysql_client = db::sql_client::MySqlClient::new(&env::var("DATABASE_URL").unwrap()).await;

    let tables = mysql_client
        .get_table_list(&env::var("DB_NAME").unwrap())
        .await;
    let default_table_name = &tables[0];

    let (headers, records) = mysql_client.get_record_list(default_table_name).await;

    let mut table_struct =
        table::TableStruct::new(default_table_name.to_string(), headers, records);

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
                            let (headers, records) =
                                mysql_client.get_record_list(selected_table_name).await;
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
    table_struct: &mut table::TableStruct,
    table_list_state: &mut ListState,
) {
    let size = f.size();

    table_struct.update_visible_range();

    let (chunks_1, chunks_2) = ui::layout::make_layout(size);

    ui::input_query::render_input_query_wdg(f, chunks_1[0]);
    ui::tables::render_table_list_wdg(f, chunks_2[0], tables, table_list_state);
    ui::records::render_table_records_wdg(f, chunks_2[1], table_struct);
}

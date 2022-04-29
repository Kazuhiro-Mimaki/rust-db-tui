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
use ui::tab;

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

    let (column_headers, column_items) = mysql_client.get_table_columns(default_table_name).await;
    let mut column_table =
        table::TableStruct::new(default_table_name.to_string(), column_headers, column_items);

    let mut tab_struct = ui::tab::TabStruct::new();

    loop {
        terminal.draw(|f| {
            render_layout(
                f,
                &tables,
                &mut tab_struct,
                &mut table_struct,
                &mut column_table,
                &mut table_list_state,
            )
        })?;

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
                        if selected < tables.len().saturating_sub(1) {
                            table_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Char('0') => {
                    tab_struct.index = 0;
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
                        }
                    }
                }
                KeyCode::Char('1') => {
                    tab_struct.index = 1;
                    if let Some(selected) = table_list_state.selected() {
                        let selected_table_name = &tables[selected];
                        if selected_table_name.to_string() != column_table.name {
                            let (headers, records) =
                                mysql_client.get_table_columns(selected_table_name).await;
                            column_table
                                .reset_default_records(
                                    selected_table_name.to_string(),
                                    headers,
                                    records,
                                )
                                .await;
                        }
                    }
                }
                KeyCode::Up => {
                    match tab_struct.index {
                        0 => {
                            table_struct.move_up();
                        }
                        1 => {
                            column_table.move_up();
                        }
                        _ => unreachable!(),
                    };
                }
                KeyCode::Down => {
                    match tab_struct.index {
                        0 => {
                            table_struct.move_down();
                        }
                        1 => {
                            column_table.move_down();
                        }
                        _ => unreachable!(),
                    };
                }
                KeyCode::Right => {
                    match tab_struct.index {
                        0 => {
                            table_struct.move_right();
                        }
                        1 => {
                            column_table.move_right();
                        }
                        _ => unreachable!(),
                    };
                }
                KeyCode::Left => {
                    match tab_struct.index {
                        0 => {
                            table_struct.move_left();
                        }
                        1 => {
                            column_table.move_left();
                        }
                        _ => unreachable!(),
                    };
                }
                KeyCode::Enter => {
                    tab_struct.index = 0;
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
    tab_struct: &mut tab::TabStruct,
    table_struct: &mut table::TableStruct,
    column_table: &mut table::TableStruct,
    table_list_state: &mut ListState,
) {
    let size = f.size();

    table_struct.update_visible_range();

    let (chunks_1, chunks_2, chunks_3) = ui::layout::make_layout(size);

    ui::input_query::render_input_query_wdg(f, chunks_1[0]);
    ui::tables::render_table_list_wdg(f, chunks_2[0], tables, table_list_state);
    ui::tab::render_tabs_wdg(f, chunks_3[0], tab_struct);
    ui::tab::render_table_by_tab_wdg(f, chunks_3[1], tab_struct, table_struct, column_table);
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use db::sql_client::MySqlClient;
use dotenv::dotenv;
use model::table::TableModel;
use std::env;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use ui::{
    layouts::{change_db::ChangeDBLayout, edit_sql::EditSQLLayout, normal::NormalLayout},
    widgets::{ctx::WidgetCtx, tab::TableMode},
};

use crate::ui::layouts::layout_trait::LayoutTrait;

mod db;
mod model;
mod ui;
mod utils;

enum WidgetMode {
    Normal,
    ChangeDB,
    EditSQL,
}

pub struct App {
    widget_mode: WidgetMode,
}

impl App {
    fn new() -> Self {
        Self {
            widget_mode: WidgetMode::Normal,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set config
    dotenv().ok();

    let db_url = format!("{}", &env::var("DATABASE_URL").unwrap());
    let mut mysql_client = MySqlClient::new(&db_url).await;
    let databases = mysql_client.get_db_list().await;
    let default_database_name = &databases[0];

    let new_db_url = format!(
        "{}/{}",
        &env::var("DATABASE_URL").unwrap(),
        default_database_name.to_string()
    );
    mysql_client.change_db(&new_db_url).await;

    let table_names = mysql_client
        .get_table_list(default_database_name.to_string())
        .await;
    let default_table_name = &table_names[0];

    let table_model = TableModel::new(&mysql_client, default_table_name.to_string()).await;

    let mut app = App::new();
    let mut widget_ctx = WidgetCtx::new(databases, table_names, table_model);

    loop {
        terminal.draw(|f| render_layout(f, &mut app, &mut widget_ctx))?;

        if let Event::Key(key) = event::read()? {
            match app.widget_mode {
                WidgetMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('a') => {
                        widget_ctx.table_list.move_up();
                    }
                    KeyCode::Char('b') => {
                        widget_ctx.table_list.move_down();
                    }
                    KeyCode::Char('e') => {
                        app.widget_mode = WidgetMode::EditSQL;
                    }
                    KeyCode::Char('c') => {
                        app.widget_mode = WidgetMode::ChangeDB;
                    }
                    KeyCode::Char('0') => {
                        widget_ctx.tab.mode = TableMode::Records;
                    }
                    KeyCode::Char('1') => {
                        widget_ctx.tab.mode = TableMode::Columns;
                    }
                    KeyCode::Up => {
                        match widget_ctx.tab.mode {
                            TableMode::Records => {
                                widget_ctx.table_record.move_up();
                            }
                            TableMode::Columns => {
                                widget_ctx.table_column.move_up();
                            }
                        };
                    }
                    KeyCode::Down => {
                        match widget_ctx.tab.mode {
                            TableMode::Records => {
                                widget_ctx.table_record.move_down();
                            }
                            TableMode::Columns => {
                                widget_ctx.table_column.move_down();
                            }
                        };
                    }
                    KeyCode::Right => {
                        match widget_ctx.tab.mode {
                            TableMode::Records => {
                                widget_ctx.table_record.move_right();
                            }
                            TableMode::Columns => {
                                widget_ctx.table_column.move_right();
                            }
                        };
                    }
                    KeyCode::Left => {
                        match widget_ctx.tab.mode {
                            TableMode::Records => {
                                widget_ctx.table_record.move_left();
                            }
                            TableMode::Columns => {
                                widget_ctx.table_column.move_left();
                            }
                        };
                    }
                    KeyCode::Enter => {
                        widget_ctx.table_list.change_table();
                        if !widget_ctx
                            .table_record
                            .is_current_table(widget_ctx.table_list.current_table.to_string())
                        {
                            // reset table records
                            let (record_headers, record_fields) = mysql_client
                                .get_table_records(widget_ctx.table_list.current_table.to_string())
                                .await;
                            widget_ctx.table_record.reset_default_records(
                                widget_ctx.table_list.current_table.to_string(),
                                record_headers,
                                record_fields,
                            );
                            // reset table columns
                            let (column_headers, column_fields) = mysql_client
                                .get_table_columns(widget_ctx.table_list.current_table.to_string())
                                .await;
                            widget_ctx.table_column.reset_default_records(
                                widget_ctx.table_list.current_table.to_string(),
                                column_headers,
                                column_fields,
                            );
                        }
                    }
                    _ => {}
                },
                WidgetMode::ChangeDB => match key.code {
                    KeyCode::Enter => {
                        widget_ctx.database.change_database();
                        // change db
                        let new_db_url = format!(
                            "{}/{}",
                            &env::var("DATABASE_URL").unwrap(),
                            widget_ctx.database.current_database.to_string()
                        );
                        mysql_client.change_db(&new_db_url).await;

                        let new_tables = mysql_client
                            .get_table_list(widget_ctx.database.current_database.to_string())
                            .await;
                        let new_table_name = &new_tables[0];
                        widget_ctx.table_list.change_table();
                        widget_ctx.table_list.change_tables(new_tables.clone());

                        // reset table records
                        let (record_headers, record_fields) = mysql_client
                            .get_table_records(new_table_name.to_string())
                            .await;
                        widget_ctx.table_record.reset_default_records(
                            widget_ctx.table_list.current_table.to_string(),
                            record_headers,
                            record_fields,
                        );

                        // reset table columns
                        let (column_headers, column_fields) = mysql_client
                            .get_table_columns(new_table_name.to_string())
                            .await;
                        widget_ctx.table_column.reset_default_records(
                            widget_ctx.table_list.current_table.to_string(),
                            column_headers,
                            column_fields,
                        );
                        app.widget_mode = WidgetMode::Normal;
                    }
                    KeyCode::Up => {
                        widget_ctx.database.move_up();
                    }
                    KeyCode::Down => {
                        widget_ctx.database.move_down();
                    }
                    KeyCode::Char('c') => {
                        app.widget_mode = WidgetMode::Normal;
                    }
                    _ => {}
                },
                WidgetMode::EditSQL => match key.code {
                    KeyCode::Enter => {
                        let res = mysql_client
                            .execute_input_query(widget_ctx.sql_input.input.clone())
                            .await;
                        match res {
                            Ok(res) => widget_ctx.sql_output.set_success_msg(res),
                            Err(e) => widget_ctx.sql_output.set_error_msg(e.to_string()),
                        }
                    }
                    KeyCode::Char(c) => {
                        widget_ctx.sql_input.input.push(c);
                    }
                    KeyCode::Backspace => {
                        widget_ctx.sql_input.input.pop();
                    }
                    KeyCode::Esc => {
                        app.widget_mode = WidgetMode::Normal;
                    }
                    _ => {}
                },
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

fn render_layout<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, widget_ctx: &mut WidgetCtx) {
    let size = f.size();
    widget_ctx.table_record.update_visible_range();

    match app.widget_mode {
        WidgetMode::Normal => {
            let normal_layout = NormalLayout::new(size);
            normal_layout.render_layout(f, widget_ctx);
        }
        WidgetMode::ChangeDB => {
            let change_db_layout = ChangeDBLayout::new(size);
            change_db_layout.render_layout(f, widget_ctx);
        }
        WidgetMode::EditSQL => {
            let edit_sql_layout = EditSQLLayout::new(size);
            edit_sql_layout.render_layout(f, widget_ctx);
        }
    }
}

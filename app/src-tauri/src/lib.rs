use todo_core::db;
mod position;

struct CommandError(serde_json::Value);
type CommandResult = std::result::Result<serde_json::Value, CommandError>;

fn to_response(result: CommandResult) -> String {
    match result {
        Ok(data) => format!(r#"{{"data": {}}}"#, data.to_string()),
        Err(error) => format!(r#"{{"error": {}}}"#, error.0.to_string()),
    }
}

impl From<db::DBError> for CommandError {
    fn from(error: db::DBError) -> Self {
        CommandError(serde_json::json!(error.to_string()))
    }
}

fn get_tasks_impl() -> CommandResult {
    let conn = db::create_connection()?;
    let tasks = db::list_tasks(&conn)?;
    Ok(serde_json::json!(tasks))
}

#[tauri::command]
fn get_tasks() -> String {
    to_response(get_tasks_impl())
}

fn post_task_impl(task: &str) -> CommandResult {
    let conn = db::create_connection()?;
    let tasks = db::insert_task(&conn, task)?;
    Ok(serde_json::json!(tasks))
}
#[tauri::command]
fn post_task(task: &str) -> String {
    to_response(post_task_impl(task))
}

fn post_task_done_impl(id: i64) -> CommandResult {
    let conn = db::create_connection()?;
    let tasks = db::done_task(&conn, id)?;
    Ok(serde_json::json!(tasks))
}
#[tauri::command]
fn post_task_done(id: i64) -> String {
    to_response(post_task_done_impl(id))
}

fn setup_app(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let tray = tauri::tray::TrayIconBuilder::new();
    let tray = tray.icon(app.default_window_icon().ok_or("cannot find icon")?.clone());

    let tray = tray.menu(&tauri::menu::Menu::with_items(
        app,
        &[&tauri::menu::MenuItem::with_id(
            app,
            "quit",
            "quit",
            true,
            None::<&str>,
        )?],
    )?);

    let handler = app.handle().clone();
    let on_tray_icon_event = move |event: tauri::tray::TrayIconEvent| match event {
        tauri::tray::TrayIconEvent::Click {
            button,
            button_state,
            position,
            ..
        } => match (button, button_state) {
            (tauri::tray::MouseButton::Left, tauri::tray::MouseButtonState::Up) => {
                match tauri::Manager::get_webview_window(&handler, "main") {
                    Some(window) => {
                        position::set_webview_windows_to_position(&window, &handler, &position);
                        window.set_focus().unwrap_or_else(|e| {
                            eprintln!("Error focusing main window: {}", e);
                        });
                    }
                    None => {
                        tauri::webview::WebviewWindowBuilder::from_config(
                            &handler,
                            &handler.config().app.windows.get(0).unwrap().clone(),
                        )
                        .and_then(tauri::webview::WebviewWindowBuilder::build)
                        .expect("cannot re-create main window");
                    }
                };
            }
            _ => {}
        },
        _ => {}
    };
    let tray = tray.show_menu_on_left_click(false);
    let tray = tray.on_tray_icon_event(move |_tray_icon, event| on_tray_icon_event(event));

    let on_menu_event = |event: tauri::menu::MenuEvent| match &event.id.0.as_str() {
        &"quit" => {
            std::process::exit(0);
        }
        _ => {} // Handle other cases if necessary
    };
    let tray = tray.on_menu_event(move |_tray_icon, event| on_menu_event(event));

    let _ = tray.build(app);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            post_task,
            post_task_done
        ])
        .setup(setup_app)
        .build(tauri::generate_context!())
        .expect("error while init application")
        .run(|_app, event| match event {
            tauri::RunEvent::Ready => {
                println!("application is ready!");
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

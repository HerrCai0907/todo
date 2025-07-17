use todo_core::db;

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

fn get_todo_list_impl() -> CommandResult {
    let conn = db::create_connection()?;
    let tasks = db::list_tasks(&conn)?;
    Ok(serde_json::json!(tasks))
}

#[tauri::command]
fn get_todo_list() -> String {
    to_response(get_todo_list_impl())
}

fn setup_app(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let tray = tauri::tray::TrayIconBuilder::new();
    let tray = tray.icon(app.default_window_icon().ok_or("cannot find icon")?.clone());

    let tray = tray.menu(&tauri::menu::Menu::with_items(
        app,
        &[&tauri::menu::MenuItem::with_id(
            app,
            "quit",
            "Quit",
            true,
            None::<&str>,
        )?],
    )?);

    let handler = app.handle().clone();
    let on_tray_icon_event = move |event: tauri::tray::TrayIconEvent| match event {
        tauri::tray::TrayIconEvent::Click {
            button,
            button_state,
            ..
        } => match (button, button_state) {
            (tauri::tray::MouseButton::Left, tauri::tray::MouseButtonState::Up) => {
                match tauri::Manager::get_webview_window(&handler, "main") {
                    Some(window) => match window.set_focus() {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error focusing window: {}", e);
                        }
                    },
                    None => {
                        tauri::webview::WebviewWindowBuilder::new(
                            &handler,
                            "main",
                            tauri::WebviewUrl::App("index.html".into()),
                        )
                        .build()
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

    let _ = tray.build(app);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_todo_list])
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

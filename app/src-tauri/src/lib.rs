use tauri::Emitter;

mod position;

struct AppData {
    kv_cache: std::collections::BTreeMap<String, String>,
    kv_watchers: std::collections::BTreeSet<String>,
}
impl AppData {
    fn new() -> Self {
        AppData {
            kv_cache: std::collections::BTreeMap::new(),
            kv_watchers: std::collections::BTreeSet::new(),
        }
    }
    fn update_kv(self: &mut Self, app_handle: tauri::AppHandle, key: String, value: String) {
        if self.kv_watchers.contains(&key) {
            app_handle
                .emit("kv_changed", &key)
                .expect("cannot emit event");
        }
        self.kv_cache.insert(key, value);
    }
}

struct CommandError(serde_json::Value);
impl From<todo_core::db::DBError> for CommandError {
    fn from(error: todo_core::db::DBError) -> Self {
        CommandError(serde_json::json!(error.to_string()))
    }
}
impl From<todo_core::root_path::Error> for CommandError {
    fn from(error: todo_core::root_path::Error) -> Self {
        CommandError(serde_json::json!(error.to_string()))
    }
}
impl From<std::io::Error> for CommandError {
    fn from(error: std::io::Error) -> Self {
        CommandError(serde_json::json!(error.to_string()))
    }
}

type CommandResult = std::result::Result<serde_json::Value, CommandError>;
fn to_response(result: CommandResult) -> String {
    match result {
        Ok(data) => format!(r#"{{"data": {}}}"#, data.to_string()),
        Err(error) => format!(r#"{{"error": {}}}"#, error.0.to_string()),
    }
}

fn get_tasks_impl() -> CommandResult {
    let conn = todo_core::db::create_connection()?;
    let tasks = todo_core::db::list_tasks(&conn)?;
    Ok(serde_json::json!(tasks))
}

#[tauri::command]
fn get_tasks() -> String {
    to_response(get_tasks_impl())
}

fn put_task_impl(task: &str) -> CommandResult {
    let conn = todo_core::db::create_connection()?;
    let tasks = todo_core::db::insert_task(&conn, task)?;
    Ok(serde_json::json!(tasks))
}
#[tauri::command]
fn put_task(task: &str) -> String {
    to_response(put_task_impl(task))
}

fn patch_task_status_done_impl(id: i64) -> CommandResult {
    let conn = todo_core::db::create_connection()?;
    todo_core::db::done_task(&conn, id)?;
    Ok(serde_json::json!(()))
}
#[tauri::command]
fn patch_task_status_done(id: i64) -> String {
    to_response(patch_task_status_done_impl(id))
}

fn patch_task_task_impl(id: i64, task: &str) -> CommandResult {
    let conn = todo_core::db::create_connection()?;
    todo_core::db::edit_task(&conn, id, &task.to_string())?;
    Ok(serde_json::json!(()))
}
#[tauri::command]
fn patch_task_task(id: i64, task: &str) -> String {
    to_response(patch_task_task_impl(id, task))
}

#[tauri::command]
fn register_event_on_storage_change(
    state: tauri::State<std::sync::Mutex<AppData>>,
    key: &str,
) -> String {
    return to_response(|| -> CommandResult {
        state.lock().unwrap().kv_watchers.insert(key.to_string());
        Ok(serde_json::json!(()))
    }());
}

#[tauri::command]
fn get_storage(state: tauri::State<std::sync::Mutex<AppData>>, key: &str) -> String {
    to_response(|| -> CommandResult {
        if let Some(value) = state.lock().unwrap().kv_cache.get(key) {
            return Ok(serde_json::json!(value));
        }
        let file = std::path::PathBuf::from(todo_core::root_path::get_folder()?).join(key);
        Ok(std::fs::read_to_string(file)
            .map_or_else(|_| serde_json::json!(()), |value| serde_json::json!(value)))
    }())
}

#[tauri::command]
fn set_storage(
    app_handle: tauri::AppHandle,
    state: tauri::State<std::sync::Mutex<AppData>>,
    key: &str,
    value: &str,
) -> String {
    to_response(move || -> CommandResult {
        let file = std::path::PathBuf::from(todo_core::root_path::get_folder()?).join(key);
        std::fs::write(file, value)?;
        state
            .lock()
            .unwrap()
            .update_kv(app_handle, key.to_string(), value.to_string());
        Ok(serde_json::json!(()))
    }())
}

fn setup_app(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    tauri::Manager::manage(app, std::sync::Mutex::new(AppData::new()));
    let tray = tauri::tray::TrayIconBuilder::new();
    let tray = tray.icon(app.default_window_icon().ok_or("cannot find icon")?.clone());

    let tray = tray.menu(&tauri::menu::Menu::with_items(
        app,
        &[
            &tauri::menu::MenuItem::with_id(app, "quit", "quit", true, None::<&str>)?,
            &tauri::menu::MenuItem::with_id(app, "config", "config", true, None::<&str>)?,
        ],
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
                let window = match tauri::Manager::get_webview_window(&handler, "main") {
                    Some(window) => window,
                    None => tauri::webview::WebviewWindowBuilder::from_config(
                        &handler,
                        &handler.config().app.windows.get(0).unwrap().clone(),
                    )
                    .and_then(tauri::webview::WebviewWindowBuilder::build)
                    .expect("cannot re-create main window"),
                };
                position::set_webview_windows_to_position(&window, &handler, &position);
                window.set_focus().unwrap_or_else(|e| {
                    eprintln!("error focusing main window: {}", e);
                });
            }
            _ => {}
        },
        _ => {}
    };
    let tray = tray.show_menu_on_left_click(false);
    let tray = tray.on_tray_icon_event(move |_tray_icon, event| on_tray_icon_event(event));

    let handler = app.handle().clone();
    let on_menu_event = move |event: tauri::menu::MenuEvent| match &event.id.0.as_str() {
        &"quit" => {
            std::process::exit(0);
        }
        &"config" => {
            let window = match tauri::Manager::get_webview_window(&handler, "config") {
                Some(window) => window,
                None => tauri::webview::WebviewWindowBuilder::from_config(
                    &handler,
                    &handler.config().app.windows.get(1).unwrap().clone(),
                )
                .and_then(tauri::webview::WebviewWindowBuilder::build)
                .expect("cannot create config window"),
            };
            window.set_focus().unwrap_or_else(|e| {
                eprintln!("error focusing config window: {}", e);
            });
        }
        _ => todo!(), // Handle other cases if necessary
    };
    let tray = tray.on_menu_event(move |_tray_icon, event| on_menu_event(event));

    let _ = tray.build(app);
    Ok(())
}

fn init_database() -> Result<(), todo_core::db::DBError> {
    let conn = todo_core::db::create_connection()?;
    todo_core::db::ensure_table(&conn)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_database().expect("cannot init database");
    let plugin = tauri_plugin_log::Builder::new().targets([
        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
    ]);

    let plugin = match todo_core::root_path::get_folder() {
        Err(_) => plugin,
        Ok(dir) => plugin.target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: std::path::PathBuf::from(dir),
                file_name: None,
            },
        )),
    };
    let plugin = plugin.build();
    tauri::Builder::default()
        .plugin(plugin)
        .invoke_handler(tauri::generate_handler![
            // task
            get_tasks,
            put_task,
            patch_task_status_done,
            patch_task_task,
            // render order
            set_storage,
            get_storage,
            //
            register_event_on_storage_change,
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

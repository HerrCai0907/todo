use todo_core::db;

fn list_task() -> Result<Vec<db::OpenTask>, db::DBError> {
    let conn = db::create_connection()?;
    let tasks = db::list_tasks(&conn)?;
    Ok(tasks)
}

#[tauri::command]
fn get_todo_list() -> String {
    match list_task() {
        Ok(tasks) => match serde_json::to_string(&tasks) {
            Ok(json) => format!(
                r#"
            {{"data": {}}}
            "#,
                json
            ),
            Err(e) => {
                eprintln!("{}", e);
                let e = serde_json::to_string(&e.to_string())
                    .unwrap_or_else(|_| "unknown error".to_string());
                format!(
                    r#"
            {{"err": {}}}
            "#,
                    e
                )
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            let e = serde_json::to_string(&e.to_string())
                .unwrap_or_else(|_| "unknown error".to_string());
            format!(
                r#"
            {{"err": {}}}
            "#,
                e
            )
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_todo_list])
        .setup(|app| {
            let quit_item =
                tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&quit_item])?;
            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().ok_or("cannot find icon")?.clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .build(app);
            Ok(())
        })
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

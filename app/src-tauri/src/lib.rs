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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_todo_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

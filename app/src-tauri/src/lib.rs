use todo_core::db;

fn list_task() -> Result<Vec<db::OpenTask>, db::DBError> {
    let conn = db::create_connection()?;
    let tasks = db::list_tasks(&conn)?;
    Ok(tasks)
}

#[tauri::command]
fn get_todo_list() -> String {
    let s = match list_task() {
        Ok(tasks) => {
            dbg!(&tasks);
            format!("Todo list: {:?}", tasks)
        }
        Err(e) => {
            dbg!(&e);
            format!("Error: {}", e)
        }
    };
    s
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_todo_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

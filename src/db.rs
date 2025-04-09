use rusqlite::Connection;

pub fn create_connection() -> rusqlite::Result<Connection> {
    Connection::open("todo.db")
}

pub fn ensure_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        r##"
      CREATE TABLE IF NOT EXISTS todo
      (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          create_time TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime')),
          finished_time TIMESTAMP,
          task TEXT NOT NULL,
          status TEXT NOT NULL CHECK (status IN ('open', 'closed', 'deleted')) DEFAULT 'open'
      )
      "##,
        [],
    )?;
    Ok(())
}

pub fn insert_task(conn: &Connection, task: &str) -> rusqlite::Result<()> {
    conn.execute(
        r##"
        INSERT INTO todo (task) VALUES (?1)
        "##,
        [&task],
    )?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> rusqlite::Result<()> {
    conn.execute(
        r##"
        UPDATE todo SET status = 'deleted' WHERE id = ?1
        "##,
        [id],
    )?;
    Ok(())
}

pub struct OpenTask {
    pub id: i64,
    pub create_time: String,
    pub task: String,
}
impl OpenTask {
    fn new(id: i64, create_time: String, task: String) -> Self {
        Self {
            id,
            create_time,
            task,
        }
    }
}

pub fn list_tasks(conn: &Connection) -> rusqlite::Result<Vec<OpenTask>> {
    let mut stmt = conn.prepare(
        r##"
        SELECT id, create_time, task FROM todo WHERE status = 'open'
        "##,
    )?;
    let ret = stmt
        .query_map([], |row| {
            Ok(OpenTask::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?
        .collect();
    ret
}

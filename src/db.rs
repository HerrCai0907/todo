use rusqlite::Connection;
use snafu::{prelude::Snafu, ResultExt};

#[derive(Debug, Snafu)]
pub enum DBError {
    #[snafu(display("cannot find environment '{}'", env))]
    Env {
        source: std::env::VarError,
        env: String,
    },
    #[snafu(display("failed to create directory in '{}'", path))]
    CreateDir {
        source: std::io::Error,
        path: String,
    },
    #[snafu(display("failed to connect to database in '{}'", db_path))]
    Connect {
        source: rusqlite::Error,
        db_path: String,
    },
    #[snafu(display("failed to run sql '{}'", sql))]
    Sql {
        source: rusqlite::Error,
        sql: &'static str,
    },
}

type Result<T> = std::result::Result<T, DBError>;

pub fn create_connection() -> Result<Connection> {
    let home_path = std::env::var("HOME").context(EnvSnafu { env: "HOME" })?;
    let dir_path = format!("{}/.todo", home_path);
    std::fs::create_dir_all(&dir_path).context(CreateDirSnafu { path: dir_path })?;
    let db_path = format!("{}/.todo/todo.db", home_path);
    let conn = Connection::open(&db_path).context(ConnectSnafu { db_path })?;
    Ok(conn)
}

pub fn ensure_table(conn: &Connection) -> Result<()> {
    let sql = r##"
    CREATE TABLE IF NOT EXISTS todo
    (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        create_time TIMESTAMP NOT NULL DEFAULT (DATETIME('now', 'localtime')),
        finished_time TIMESTAMP,
        task TEXT NOT NULL,
        status TEXT NOT NULL CHECK (status IN ('open', 'closed', 'deleted')) DEFAULT 'open'
    )
    "##;
    conn.execute(sql, []).context(SqlSnafu { sql })?;
    Ok(())
}

pub fn insert_task(conn: &Connection, task: &str) -> Result<()> {
    let sql = r##"
    INSERT INTO todo (task) VALUES (?1)
    "##;
    conn.execute(sql, [&task]).context(SqlSnafu { sql })?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> Result<()> {
    let sql = r##"
UPDATE todo SET status = 'deleted' WHERE id = ?1
"##;
    conn.execute(sql, [id]).context(SqlSnafu { sql })?;
    Ok(())
}

pub struct OpenTask {
    pub id: i64,
    pub create_time: String,
    pub task: String,
}
impl OpenTask {
    fn new(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            create_time: row.get(1)?,
            task: row.get(2)?,
        })
    }
}

pub fn list_tasks(conn: &Connection) -> Result<Vec<OpenTask>> {
    let sql = r##"
        SELECT id, create_time, task FROM todo WHERE status = 'open'
    "##;
    let ret: Vec<OpenTask> = (|| -> rusqlite::Result<Vec<OpenTask>> {
        let mut stmt = conn.prepare(sql)?;
        let ret = stmt
            .query_map([], |row: &rusqlite::Row<'_>| Ok(OpenTask::new(row)?))?
            .collect();
        ret
    })()
    .context(SqlSnafu { sql })?;
    Ok(ret)
}

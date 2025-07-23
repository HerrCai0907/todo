use snafu::{ResultExt, prelude::Snafu};

pub struct Connection(rusqlite::Connection);

impl Connection {
    pub fn execute<P: rusqlite::Params>(&self, sql: &str, params: P) -> rusqlite::Result<usize> {
        self.0.execute(sql, params)
    }
    fn prepare(&self, sql: &str) -> rusqlite::Result<rusqlite::Statement<'_>> {
        self.0.prepare(sql)
    }
}

#[derive(Debug, Snafu)]
pub enum DBError {
    #[snafu(display("failed to create todo root directory"))]
    Root { source: crate::root_path::Error },
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
    #[snafu(display("failed to parse sql result '{}'", result))]
    ParseSqlResult {
        source: rusqlite::Error,
        result: String,
    },
    #[snafu(display("invalid database"))]
    InvalidDatabase {},
}

type Result<T> = std::result::Result<T, DBError>;

pub fn create_connection() -> Result<Connection> {
    let db_path = format!(
        "{}/todo.db",
        crate::root_path::get_folder().context(RootSnafu {})?
    );
    let conn = rusqlite::Connection::open(&db_path).context(ConnectSnafu { db_path })?;
    Ok(Connection(conn))
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
    INSERT INTO todo
    (task) VALUES (?1)
    "##;
    conn.execute(sql, [&task]).context(SqlSnafu { sql })?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> Result<()> {
    let sql = r##"
    UPDATE todo
    SET status = 'deleted'
    WHERE id = ?1
    "##;
    conn.0.execute(sql, [id]).context(SqlSnafu { sql })?;
    Ok(())
}

pub fn edit_task(conn: &Connection, id: i64, new_task: &String) -> Result<()> {
    let sql = r##"
    UPDATE todo
    SET task = ?2
    WHERE id = ?1
    "##;
    conn.execute(sql, rusqlite::params![id, new_task])
        .context(SqlSnafu { sql })?;
    Ok(())
}

pub fn done_task(conn: &Connection, id: i64) -> Result<()> {
    let sql = r##"
    UPDATE todo
    SET status = 'closed',
        finished_time = DATETIME('now', 'localtime')
    WHERE id = ?1
    "##;
    conn.execute(sql, [id]).context(SqlSnafu { sql })?;
    Ok(())
}

pub fn clean_outdate_task(conn: &Connection) -> Result<()> {
    let sql = r##"
        DELETE FROM todo
        WHERE (status = 'closed' OR status = 'deleted') AND
            (finished_time IS NULL OR finished_time <= DATETIME('now', 'localtime', '-1 weeks'))
    "##;
    conn.execute(sql, []).context(SqlSnafu { sql })?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
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
            .query_map([], |row: &rusqlite::Row<'_>| OpenTask::new(row))?
            .collect();
        ret
    })()
    .context(SqlSnafu { sql })?;
    Ok(ret)
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Open,
    Closed,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub create_time: String,
    pub finished_time: Option<String>,
    pub task: String,
    pub status: TaskStatus,
}

#[derive(Clone)]
struct TaskImpl {
    pub id: i64,
    pub create_time: String,
    pub finished_time: Option<String>,
    pub task: String,
    pub status: String,
}

impl TaskImpl {
    fn new(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            create_time: row.get(1)?,
            finished_time: row.get(2)?,
            status: row.get(3)?,
            task: row.get(4)?,
        })
    }
    fn to_task(self: &Self) -> Result<Task> {
        Ok(Task {
            id: self.id,
            create_time: self.create_time.clone(),
            finished_time: self.finished_time.clone(),
            status: match self.status.as_str() {
                "open" => TaskStatus::Open,
                "closed" => TaskStatus::Closed,
                "deleted" => TaskStatus::Deleted,
                _ => panic!(),
            },
            task: self.task.clone(),
        })
    }
}

pub fn list_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let sql = r##"
        SELECT id, create_time, finished_time, status, task FROM todo
    "##;

    let mut stmt = conn.prepare(sql).context(SqlSnafu { sql })?;
    let ret = stmt
        .query_map([], |row: &rusqlite::Row<'_>| TaskImpl::new(row))
        .context(SqlSnafu { sql })?
        .collect::<rusqlite::Result<Vec<TaskImpl>>>()
        .context(SqlSnafu { sql })?;
    let ret = ret
        .iter()
        .map(|task_impl| TaskImpl::to_task(task_impl))
        .collect();
    ret
}

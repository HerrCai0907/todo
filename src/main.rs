mod db;
mod interaction;

use clap::Parser;
use db::DBError;
use interaction::InteractionError;
use snafu::{prelude::Snafu, ResultExt};

#[derive(Debug, Snafu)]
pub enum TodoError {
    #[snafu(display("database error when {}", cases))]
    Database {
        source: DBError,
        cases: &'static str,
    },
    #[snafu(display("user interaction error"))]
    Interaction {
        source: InteractionError,
        cases: &'static str,
    },
    #[snafu(display("invalid input '{}', expected '{}'", input, expect))]
    Input { input: String, expect: &'static str },
    #[snafu(display("operator cancelled by user"))]
    UserCancelled,
}

type TodoResult<T> = std::result::Result<T, TodoError>;

fn add_task() -> TodoResult<()> {
    let task = interaction::read_input("task");
    if task.is_empty() {
        return Err(TodoError::Input {
            input: task,
            expect: "string task",
        });
    }
    let conn = db::create_connection().context(DatabaseSnafu { cases: "add task" })?;
    db::ensure_table(&conn).context(DatabaseSnafu { cases: "add task" })?;
    db::insert_task(&conn, &task).context(DatabaseSnafu { cases: "add task" })?;
    list_tasks()?;
    Ok(())
}

fn select_task(conn: &rusqlite::Connection) -> TodoResult<Option<db::OpenTask>> {
    db::ensure_table(&conn).context(DatabaseSnafu {
        cases: "select task",
    })?;
    let tasks = db::list_tasks(&conn).context(DatabaseSnafu {
        cases: "select task",
    })?;
    let mut task_names: Vec<&str> = tasks.iter().map(|x| x.task.as_str()).collect();
    let cancel_index = task_names.len();
    task_names.push("cancel");
    let index = interaction::select(&task_names).context(InteractionSnafu {
        cases: "select task",
    })?;
    Ok(if index == cancel_index {
        None
    } else {
        Some(tasks[index].clone())
    })
}

fn select_and_delete_task() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu {
        cases: "delete task",
    })?;
    match select_task(&conn)? {
        Some(task) => {
            db::delete_task(&conn, task.id).context(DatabaseSnafu {
                cases: "delete task",
            })?;
            println!("delete task({}): {} ", task.id, task.task);
            Ok(())
        }
        None => Err(TodoError::UserCancelled {}),
    }
}

fn select_and_done_task() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu {
        cases: "delete task",
    })?;
    match select_task(&conn)? {
        Some(task) => {
            db::done_task(&conn, task.id).context(DatabaseSnafu { cases: "done task" })?;
            println!("done task({}): '{}'", task.id, task.task);
            Ok(())
        }
        None => Err(TodoError::UserCancelled {}),
    }
}

fn select_and_edit_task() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu { cases: "edit task" })?;
    match select_task(&conn)? {
        Some(task) => {
            let new_task: String = "".to_string();
            let old_task = task.task;
            db::edit_task(&conn, task.id, &new_task)
                .context(DatabaseSnafu { cases: "edit task" })?;
            println!(
                "edit task({}):\n\t'{}'\n\t-> '{}'",
                task.id, old_task, new_task
            );
            Ok(())
        }
        None => Err(TodoError::UserCancelled {}),
    }
}

fn list_tasks() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu { cases: "list task" })?;
    db::ensure_table(&conn).context(DatabaseSnafu { cases: "list task" })?;
    let tasks = db::list_tasks(&conn).context(DatabaseSnafu { cases: "list task" })?;
    for task in tasks {
        println!("{}({}): {}", task.id, task.create_time, task.task);
    }
    Ok(())
}

fn list_all_tasks() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu { cases: "list task" })?;
    db::ensure_table(&conn).context(DatabaseSnafu { cases: "list task" })?;
    let tasks = db::list_all_tasks(&conn).context(DatabaseSnafu { cases: "list task" })?;
    for task in tasks {
        println!(
            "{}[{}]({} - {}): {}",
            task.id,
            match task.status {
                db::TaskStatus::Open => "OPEN",
                db::TaskStatus::Closed => "CLOSE",
                db::TaskStatus::Deleted => "DELETE",
            },
            task.create_time,
            match task.finished_time {
                Some(finished_time) => finished_time,
                None => "".to_owned(),
            },
            task.task
        );
    }
    Ok(())
}

fn clean_tasks() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu {
        cases: "clean task",
    })?;
    db::ensure_table(&conn).context(DatabaseSnafu {
        cases: "clean task",
    })?;
    db::clean_outdate_task(&conn).context(DatabaseSnafu {
        cases: "clean task",
    })?;
    list_all_tasks()?;
    Ok(())
}

fn todo_main() -> TodoResult<()> {
    let m = interaction::Cli::parse();
    match m.command {
        interaction::Commands::Add {} => add_task(),
        interaction::Commands::Del {} => select_and_delete_task(),
        interaction::Commands::Done {} => select_and_done_task(),
        interaction::Commands::Edit {} => select_and_edit_task(),

        interaction::Commands::Clean {} => clean_tasks(),

        interaction::Commands::List { all } => {
            if all {
                list_all_tasks()
            } else {
                list_tasks()
            }
        }
    }
}

fn main() {
    match todo_main() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error stack:\n{}", snafu::Report::from_error(&e));
            std::process::exit(1);
        }
    }
}

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
}

type TodoResult<T> = std::result::Result<T, TodoError>;

fn add_task() -> TodoResult<()> {
    let task = interaction::read_input("task");
    if task.is_empty() {
        return Err(TodoError::Input {
            input: task,
            expect: "task",
        });
    }
    let conn = db::create_connection().context(DatabaseSnafu { cases: "add task" })?;
    db::ensure_table(&conn).context(DatabaseSnafu { cases: "add task" })?;
    db::insert_task(&conn, &task).context(DatabaseSnafu { cases: "add task" })?;
    Ok(())
}

fn select_and_delete_task() -> TodoResult<()> {
    let conn = db::create_connection().context(DatabaseSnafu {
        cases: "delete task",
    })?;
    db::ensure_table(&conn).context(DatabaseSnafu {
        cases: "delete task",
    })?;
    let tasks = db::list_tasks(&conn).context(DatabaseSnafu {
        cases: "delete task",
    })?;
    let mut task_names: Vec<&str> = tasks.iter().map(|x| x.task.as_str()).collect();
    let cancel_index = task_names.len();
    task_names.push("cancel");
    let index = interaction::select(&task_names).context(InteractionSnafu {
        cases: "delete task",
    })?;
    if index == cancel_index {
        println!("operation canceled")
    } else {
        db::delete_task(&conn, tasks[index].id).context(DatabaseSnafu {
            cases: "delete task",
        })?;
        println!(
            "delete task {} with id {}",
            tasks[index].task, tasks[index].id
        )
    };
    Ok(())
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

fn todo_main() -> TodoResult<()> {
    let m = interaction::Cli::parse();
    match m.command {
        interaction::Commands::Add {} => add_task(),
        interaction::Commands::List {} => list_tasks(),
        interaction::Commands::Del {} => select_and_delete_task(),
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

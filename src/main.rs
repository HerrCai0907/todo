use clap::Parser;
mod cli;
mod db;

fn add_task() {
    let task = cli::read_input("task");
    if task.is_empty() {
        println!("No task provided.");
        return;
    }
    let conn = db::create_connection().expect("cannot connect to database");
    db::ensure_table(&conn).expect("cannot create table");
    db::insert_task(&conn, &task).expect("cannot insert task");
}

fn list_tasks() {
    let conn = db::create_connection().expect("cannot connect to database");
    db::ensure_table(&conn).expect("cannot create table");
    let tasks = db::list_tasks(&conn);
    match tasks {
        Ok(tasks) => {
            for task in tasks {
                println!("{}({}): {}", task.id, task.create_time, task.task);
            }
        }
        Err(e) => {
            println!("Error listing tasks: {}", e);
        }
    }
}

fn main() {
    let m = cli::Cli::parse();
    match m.command {
        cli::Commands::Add {} => add_task(),
        cli::Commands::List {} => list_tasks(),
    }
}

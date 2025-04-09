use clap::{Parser, Subcommand};
use std::io::Write;

#[derive(Debug, Parser)]
#[command(propagate_version = true)]
#[command(version, about = "todo command line tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {},
    Del {},
    Done {},
}

fn read_input(content: &str) -> String {
    let mut editor = rustyline::DefaultEditor::new().unwrap();
    let readline = editor.readline(&(content.to_owned() + ": "));
    match readline {
        Ok(line) => {
            if line.is_empty() {
                line
            } else {
                line + &read_input(content)
            }
        }
        Err(_) => String::new(),
    }
}

fn add_task() -> () {
    let task = read_input("task");
    if task.is_empty() {
        println!("No task provided.");
        return;
    }
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();
    writeln!(file, "- [] {}", task).unwrap();
    println!("Task added: {}", task);
}

fn main() {
    let m = Cli::parse();
    match m.command {
        Commands::Add {} => add_task(),
        Commands::Del {} => todo!(),
        Commands::Done {} => todo!(),
    }
}

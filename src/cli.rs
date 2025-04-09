use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(propagate_version = true)]
#[command(version, about = "todo command line tools")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add {},
    List {},
}

pub fn read_input(content: &str) -> String {
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

use clap::{Parser, Subcommand};
use crossterm::{self, execute};
use std::io;
use std::sync::mpsc;
use std::thread;

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
    Del {},
    List {},
}

pub fn read_input(content: &str) -> String {
    let mut editor = rustyline::DefaultEditor::new().unwrap();
    let readline = editor.readline(&(content.to_owned() + ">> "));
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

fn render_menu(
    stdout: &mut io::Stdout,
    options: &[&str],
    selected_index: usize,
) -> anyhow::Result<()> {
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
    )?;
    for (i, option) in options.iter().enumerate() {
        if i == selected_index {
            execute!(
                stdout,
                crossterm::cursor::MoveTo(0, i as u16),
                crossterm::style::SetForegroundColor(crossterm::style::Color::Blue),
                crossterm::style::Print(format!("> {}\n", *option)),
                crossterm::style::ResetColor,
            )?;
        } else {
            execute!(
                stdout,
                crossterm::cursor::MoveTo(0, i as u16),
                crossterm::style::Print(format!("  {}\n", *option)),
            )?;
        }
    }
    execute!(stdout, crossterm::cursor::MoveTo(0, options.len() as u16))?;
    Ok(())
}

pub fn select(options: &[&str]) -> anyhow::Result<usize> {
    assert!(!options.is_empty());
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let mut selected_index = 0;
    render_menu(&mut stdout, &options, selected_index)?;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Ok(event) = crossterm::event::read() {
            tx.send(event).unwrap();
        }
    });
    loop {
        match rx.recv()? {
            crossterm::event::Event::Key(key_event) => {
                if key_event.is_press() {
                    match key_event.code {
                        crossterm::event::KeyCode::Up => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                            render_menu(&mut stdout, &options, selected_index)?;
                        }
                        crossterm::event::KeyCode::Down => {
                            if selected_index < options.len() - 1 {
                                selected_index += 1;
                            }
                            render_menu(&mut stdout, &options, selected_index)?;
                        }
                        crossterm::event::KeyCode::Enter => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            crossterm::event::Event::Resize(_, _) => {
                render_menu(&mut stdout, &options, selected_index)?;
            }
            _ => {}
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    execute!(
        stdout,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show,
    )?;
    Ok(selected_index)
}

use clap::{Parser, Subcommand};
use crossterm::{self, execute};
use snafu::{prelude::Snafu, ResultExt};

#[derive(Debug, Snafu)]
pub enum InteractionError {
    #[snafu(display("terminal error when '{}'", operator))]
    Terminal {
        source: std::io::Error,
        operator: &'static str,
    },
    #[snafu(display("mpsc communication failed"))]
    RecvEvent { source: std::sync::mpsc::RecvError },
}

type Result<T> = std::result::Result<T, InteractionError>;

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
    Edit {},
    Done {},
    List {
        #[arg(long)]
        all: bool,
    },
    Clean {},
}

type LineEditor = rustyline::Editor<(), rustyline::history::FileHistory>;

fn read_line_impl(editor: &mut LineEditor, prompt: &str) -> String {
    let readline = editor.readline(prompt);
    match readline {
        Ok(line) => {
            if line.is_empty() {
                line
            } else {
                line + &read_line_impl(editor, prompt)
            }
        }
        Err(_) => String::new(),
    }
}

pub fn read_input(content: &str) -> String {
    let mut editor: LineEditor = rustyline::DefaultEditor::new().unwrap();
    read_line_impl(&mut editor, &(content.to_owned() + ">> "))
}

pub fn read_input_with_default(content: &str, default: &str) -> String {
    let mut editor: LineEditor = rustyline::DefaultEditor::new().unwrap();
    let prompt: String = content.to_owned() + ">> ";
    match editor.readline_with_initial(&prompt, (default, "")) {
        Ok(line) => {
            if line.is_empty() {
                String::new()
            } else {
                line + &read_line_impl(&mut editor, &prompt)
            }
        }
        Err(_) => String::new(),
    }
}

fn render_menu(
    stdout: &mut std::io::Stdout,
    options: &[&str],
    selected_index: usize,
) -> Result<()> {
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
    )
    .context(TerminalSnafu {
        operator: "clear screen",
    })?;
    for (i, option) in options.iter().enumerate() {
        if i == selected_index {
            execute!(
                stdout,
                crossterm::cursor::MoveTo(0, i as u16),
                crossterm::style::SetForegroundColor(crossterm::style::Color::Blue),
                crossterm::style::Print(format!("> {}\n", *option)),
                crossterm::style::ResetColor,
            )
            .context(TerminalSnafu {
                operator: "render menu",
            })?;
        } else {
            execute!(
                stdout,
                crossterm::cursor::MoveTo(0, i as u16),
                crossterm::style::Print(format!("  {}\n", *option)),
            )
            .context(TerminalSnafu {
                operator: "render menu",
            })?;
        }
    }
    execute!(stdout, crossterm::cursor::MoveTo(0, options.len() as u16)).context(
        TerminalSnafu {
            operator: "reset cursor",
        },
    )?;
    Ok(())
}

pub fn select(options: &[&str]) -> Result<usize> {
    assert!(!options.is_empty());
    crossterm::terminal::enable_raw_mode().context(TerminalSnafu {
        operator: "enable_raw_mode",
    })?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )
    .context(TerminalSnafu {
        operator: "enter alternate screen",
    })?;
    let mut selected_index = 0;
    render_menu(&mut stdout, &options, selected_index)?;

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || loop {
        if let Ok(event) = crossterm::event::read() {
            match tx.send(event) {
                Ok(_) => {}
                Err(_) => break,
            }
        }
    });
    loop {
        match rx.recv().context(RecvEventSnafu {})? {
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
    crossterm::terminal::disable_raw_mode().context(TerminalSnafu {
        operator: "disable_raw_mode",
    })?;
    execute!(
        stdout,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show,
    )
    .context(TerminalSnafu {
        operator: "leave alternate screen",
    })?;
    Ok(selected_index)
}

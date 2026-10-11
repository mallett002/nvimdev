use std::env;
use std::fs;
use std::io::{self, Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let projects = get_projects()?;

    select_project(projects)?;

    Ok(())
}

fn select_project(projects: Vec<String>) -> Result<(), std::io::Error> {
    let mut selected_index = 0;

    enable_raw_mode()?; // disable terminal default behavior
    let mut stdout = stdout(); // allows terminal to move cursor

    // turn off blinking terminal text cursor; get blank canvas for terminal ui:
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    loop {
        // 2. Render the selection list
        queue!(stdout, MoveTo(0, 0))?;

        for (i, project) in projects.iter().enumerate() {
            if i == selected_index {
                // Highlight the selected project
                queue!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!(" > {}\r\n", project)),
                    ResetColor
                )?;
            } else {
                // Standard project - added \r before \n
                queue!(stdout, Print(format!("   {}\r\n", project)))?;
            }
        }

        stdout.flush()?;

        // 3. Read user input events
        if let Event::Key(key) = event::read()? {
            // Filter out release events (important on Windows)
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match (key.code, key.modifiers) {
                // Move Up: Up Arrow OR Ctrl + P
                (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    } else {
                        selected_index = projects.len() - 1; // Wrap around to bottom
                    }
                }

                // Move Down: Down Arrow OR Ctrl + N
                (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                    if selected_index < projects.len() - 1 {
                        selected_index += 1;
                    } else {
                        selected_index = 0; // Wrap around to top
                    }
                }

                // Select: Enter key
                (KeyCode::Enter, _) => {
                    break;
                }

                // Optional: Exit early with Esc or Ctrl+C
                (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    execute!(stdout, LeaveAlternateScreen, Show)?;
                    println!("Selection canceled.");
                    return Ok(());
                }

                _ => {}
            }
        }
    }

    // 4. Cleanup terminal state
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    println!("You selected: {}", projects[selected_index]);

    Ok(())
}

fn get_projects() -> io::Result<Vec<String>> {
    let home = env::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home dir not found"))?;

    let code_dir = home.join("code");

    let mut projects: Vec<String> = vec![];

    for entry in fs::read_dir(code_dir)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            projects.push(entry.file_name().to_string_lossy().into_owned());
        }
    }

    projects.sort_unstable();

    Ok(projects)
}

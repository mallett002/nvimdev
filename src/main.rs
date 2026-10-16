use std::env;
use std::fs;
use std::io::{self, Write, stdout};
use std::process::Command;

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

    let prj = select_project(projects)?;

    open_project(prj.as_str());

    Ok(())
}

fn select_project(projects: Vec<String>) -> Result<String, std::io::Error> {
    let mut selected_index = 0;
    let mut query = String::new();

    enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    loop {
        // Filter projects based on the search query
        let filtered: Vec<&String> = projects
            .iter()
            .filter(|project| {
                project.to_lowercase().contains(&query.to_lowercase())
            })
            .collect();

        // Keep selected index within bounds
        if !filtered.is_empty() && selected_index >= filtered.len() {
            selected_index = filtered.len() - 1;
        }

        // Render search input and results
        queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        queue!(
            stdout,
            Print(format!("Search: {}_\r\n\r\n", query))
        )?;

        for (i, project) in filtered.iter().enumerate() {
            if i == selected_index {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!(" > {}\r\n", project)),
                    ResetColor
                )?;
            } else {
                queue!(stdout, Print(format!("   {}\r\n", project)))?;
            }
        }

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match (key.code, key.modifiers) {
                // Type to search
                (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    query.push(c);
                    selected_index = 0;
                }

                // Delete the last search character
                (KeyCode::Backspace, _) => {
                    query.pop();
                    selected_index = 0;
                }

                // Move up
                (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    if !filtered.is_empty() {
                        selected_index = if selected_index == 0 {
                            filtered.len() - 1
                        } else {
                            selected_index - 1
                        };
                    }
                }

                // Move down
                (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                    if !filtered.is_empty() {
                        selected_index = (selected_index + 1) % filtered.len();
                    }
                }

                // Select project
                (KeyCode::Enter, _) => {
                    if !filtered.is_empty() {
                        let selected = filtered[selected_index].clone();

                        disable_raw_mode()?;
                        execute!(stdout, LeaveAlternateScreen, Show)?;

                        return Ok(selected.clone());
                    }
                }

                // Cancel
                (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    execute!(stdout, LeaveAlternateScreen, Show)?;
                    return Ok(String::new());
                }

                _ => {}
            }
        }
    }
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

fn open_project(project: &str) {
    let root_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to find home directory when opening project.");
            return;
        }
    };

    let project_path = root_dir
        .join("code")
        .join(project);

    // 4. Launch Neovim directly into the project directory
    let status = Command::new("nvim")
        .current_dir(&project_path) // <-- This changes directory *only* for Neovim
        .status();

    // 5. Handle any errors if Neovim fails to start
    match status {
        Ok(s) if s.success() => {
            // Neovim closed cleanly, program exits, shell stays in original dir
        }
        Ok(s) => {
            eprintln!("Neovim exited with an error status: {}", s);
        }
        Err(e) => {
            eprintln!("Failed to launch Neovim: {}", e);
        }
    }
}

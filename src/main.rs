use std::env;
use std::fs;

fn main() {
    print_projects();
}

fn print_projects() {
    if let Some(home) = env::home_dir() {
        let mut projects: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(home.join("code")) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            projects.push(name.to_owned());
                        }
                    }
                }
            }
        }

        projects.sort_unstable();

        for p in &projects {
            println!("{p}");
        }
    }
}

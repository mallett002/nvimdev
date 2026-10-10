use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    print_projects()
}

fn print_projects() -> io::Result<()> {
    let home = env::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Home dir not found")
    })?;

    let code_dir = home.join("code");

    for entry in fs::read_dir(code_dir)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}

// fn print_projectz() {
//     if let Some(home) = env::home_dir() {
//         let mut projects: Vec<String> = Vec::new();
//
//         if let Ok(entries) = fs::read_dir(home.join("code")) {
//             for entry in entries.flatten() {
//                 if let Ok(ft) = entry.file_type() {
//                     if ft.is_dir() {
//                         if let Some(name) = entry.file_name().to_str() {
//                             projects.push(name.to_owned());
//                         }
//                     }
//                 }
//             }
//         }
//
//         projects.sort_unstable();
//
//         for p in &projects {
//             println!("{p}");
//         }
//     }
// }

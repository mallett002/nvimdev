use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let projects = list_projects()?;

    for prj in projects {
        println!("{prj}");
    }

    Ok(())
}

fn list_projects() -> io::Result<Vec<String>> {
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

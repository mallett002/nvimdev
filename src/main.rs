use std::env;
use std::fs;

fn main() {
    print_projects();
}

fn print_projects() {
    let home = env::home_dir();

    if let Some(dir) = home {
        let projects_dir = dir.join("code");

        let paths = fs::read_dir(projects_dir).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display());
        }
    }

}

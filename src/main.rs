use std::env;

fn main() {
    print_projects();
}

fn print_projects() {
    let home = env::home_dir();

    match home {
        Some(found_home) => {
            let path = found_home.join("code");

            println!("found home {path:?}");
        },

        None => panic!("No home dir found!"),
    }

}

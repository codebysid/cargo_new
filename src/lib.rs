use std::fs;
use std::process;

pub fn create_file(path: &str, content: &str) {
    fs::write(path, content).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    })
}

pub fn create_directory(path: &str) {
    fs::create_dir(path).unwrap_or_else(|err| {
        println!("{err}");
        print!("creating d");
        process::exit(1);
    })
}

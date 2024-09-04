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

pub fn get_directory_name(path: &str) -> String {
    let path_parts: Vec<&str> = path.split("/").collect();

    match path_parts.last() {
        Some(dir_name) => (*dir_name).to_string(),
        None => "No directory".to_string(),
    }
}

use cargo_init::create_directory;
use cargo_init::create_file;
use cargo_init::get_directory_name;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Not enough arguments");
        return;
    }

    if args[1] == "cargo".to_string() && args[2] == "new".to_string() {
        let base_path = &args[3];
        let mut project_path = format!("./{}", base_path);

        let parts: Vec<&str> = base_path.as_str().split("/").collect();

        if parts.len() > 1 && !base_path.contains(".") {
            project_path = format!("/{}", base_path);
        }

        create_directory(&project_path);
        create_file(
            &format!("{}/Cargo.toml", base_path),
            &format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
                get_directory_name(&project_path)
            ),
        );

        create_file(&format!("{}/.gitignore", base_path), "/target");

        create_directory(&format!("{}/src", project_path));

        create_file(
            &format!("{}/src/main.rs", project_path),
            r#"
fn main(){
    println!("Hello World");
}
        "#,
        );
    } else {
        println!("Wrong arguments. \nCorrect are : cargo new [project name]")
    }
}

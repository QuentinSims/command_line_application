use std::io;
mod file_utils;

fn main() {
    // let file_path = r#"c:\Users\QuentinSims\Documents\rustreadtest.txt"#;

    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");

    let file_path = file_path.trim();

    match file_utils::process_file(file_path) {
        Ok(contents) => println!("First line: {}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}


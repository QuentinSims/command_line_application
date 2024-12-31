mod file_utils;

fn main() {
    let file_path = r#"c:\Users\QuentinSims\Documents\rustreadtest.txt"#;

    match file_utils::process_file(file_path) {
        Ok(contents) => println!("First line: {}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}


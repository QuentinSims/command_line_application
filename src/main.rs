use std::io;
mod file_utils;
mod regrex_pattern;

fn main() {
    // let file_path = r#"c:\Users\QuentinSims\Documents\rustreadtest.txt"#;

    println!("Enter the file path:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");

    println!("Enter the regrex pattern:");
    let mut regrex_pattern = String::new();
    io::stdin()
       .read_line(&mut regrex_pattern)
       .expect("Failed to read input");

    let file_path = file_path.trim();
    let regrex_pattern = regrex_pattern.trim();

    match file_utils::process_file(file_path) {
        Ok(contents) => println!("First line: {}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }

    match regrex_pattern::validate_and_create_regex(regrex_pattern) {
        Ok(regex) =>  println!("Valid regex pattern created: {:?}", regex),
        Err(e) => eprintln!("Error: {}", e),
    }
}


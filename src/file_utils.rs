use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn process_file(file_path: &str) -> Result<String, String> {
    let file = get_file(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut buf_reader = get_buf_reader(file);
    let mut contents = String::new();
    let bytes_read = get_first_line(&mut buf_reader, &mut contents)
        .map_err(|e| format!("Failed to read first line: {}", e))?;

    if bytes_read == 0 {
        return Err("File is empty or contains no readable lines.".to_string());
    }

    Ok(contents)
}

fn get_file(file_path: &str) -> io::Result<File> {
    File::open(file_path)
}

fn get_buf_reader(file: File) -> BufReader<File> {
    BufReader::new(file)
}

fn get_first_line(buf_reader: &mut BufReader<File>, contents: &mut String) -> io::Result<usize> {
    buf_reader.read_line(contents)
}

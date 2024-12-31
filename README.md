Rust File Search Utility
A Rust-based utility for file searching and pattern matching, inspired by the Unix grep command. This project demonstrates the use of Rust features like file I/O, regular expressions, and command-line argument parsing in a step-by-step manner, progressing from basic functionality to advanced features.

Features (Current Progress)
Step 1: Basic File I/O
List File Metadata:

Open a file using std::fs.
Print the file name and size (metadata only).
Read the First Line:

Use a BufReader to read and display the first line of the file.
Example Output
Given a file named example.txt:

mathematica
Copy code
First Line: This is the first line of the file.
Usage
Prerequisites
Rust installed on your system. Install Rust
Run the Program
Clone the repository and execute the program with the desired file path:

bash
Copy code
cargo run
When prompted, enter the path to the file you want to read. For example:

text
Copy code
Enter file path: /path/to/your/file.txt
Next Steps
The upcoming steps include adding support for regular expression input, advanced search and filter capabilities, and command-line argument parsing.
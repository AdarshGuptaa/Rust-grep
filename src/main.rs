use std::{env, fs};

fn main() {
    let args : Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for: {query}");
    println!("In: {file_path}");

    let file_content = fs::read_to_string(file_path).expect("A readable file");

    println!("The file content:\n{file_content}");
}

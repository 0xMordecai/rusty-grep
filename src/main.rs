use std::{env, fs};

fn main() {
    // allows your minigrep program to read any command line arguments passed to it and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text \n{contents}");
}

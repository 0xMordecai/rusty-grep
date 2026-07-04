use std::env;

fn main() {
    // allows your minigrep program to read any command line arguments passed to it and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

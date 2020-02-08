use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    fs::rename(&args[1], &args[2]).unwrap();
}

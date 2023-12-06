use std::env;

mod day1;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "day1" => day1::run(),
        _ => return
    }
}



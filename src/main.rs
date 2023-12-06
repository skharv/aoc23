use std::env;

mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "day1" => day1::run(),
        "day2" => day2::run(),
        "day3" => day3::run(),
        _ => return
    }
}



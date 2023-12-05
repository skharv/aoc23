use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const RADIX: u32 = 10;

fn main() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut total = 0;
        for line in lines {
            if let Ok(text) = line {
                print!("{} - ", text);
                let mut first_found = false;
                let mut last: u32 = 0;
                for char in text.chars() {
                    if char.is_digit(RADIX) {
                        if let Some(first) = char.to_digit(RADIX) {
                            if !first_found {
                                total += first;
                                print!("{}/", first);
                            }
                            first_found = true;
                            last = first;
                        }
                    }
                }
                println!("{}", last);
                total += last;
            }
        }
        println!("{}", total);
    } else {
        println!("file not found");
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

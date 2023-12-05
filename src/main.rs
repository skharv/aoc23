use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/input") {
        let mut total = 0;
        for line in lines {
            if let Ok(text) = line {
                print!("{} - ", text);
                let integer = get_first_and_last_number(text);
                total += integer;
                println!("{}", integer);
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

fn get_first_and_last_number(text: String) -> u32 {
    let array: [&str; 19] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; 
    let mut first: (usize, u32) = (text.len(), 0); 
    let mut last: (usize, u32) = (0, 0); 
    for (char_index, _) in text.chars().enumerate() {
        let end = &text[char_index..];
        for number in array.iter() {
            if let Some(index) = end.find(number) {
                let real_index = index + char_index;
                if real_index < first.0 {
                    println!("{}: {} < {} ", number, real_index, first.0);
                    first.0 = real_index;
                    if let Ok(first_number) = number.parse::<u32>() {
                        first.1 = first_number;
                    } else {
                        first.1 = number_lookup(number);
                    }
                }

                if real_index >= last.0 {
                    println!("{}: {} >= {} ", number, real_index, last.0);
                    last.0 = real_index;
                    if let Ok(last_number) = number.parse::<u32>() {
                        last.1 = last_number;
                    } else {
                        last.1 = number_lookup(number);
                    }
                }
            }
        }
    }
    let mut output_string = String::new();
    output_string += first.1.to_string().as_str();
    output_string += last.1.to_string().as_str();

    if let Ok(output) = output_string.parse::<u32>() {
        return output;
    } else {
        println!("Unable to parse output string");
        return 0;
    }
}

fn number_lookup(text: &str) -> u32 {
    match text {
        "one" => return 1,
        "two" => return 2,
        "three" => return 3,
        "four" => return 4,
        "five" => return 5,
        "six" => return 6,
        "seven" => return 7,
        "eight" => return 8,
        "nine" => return 9,
        _ => return 0
    }
}

use super::utils;

struct Number {
    start_x: usize,
    end_x: usize,
    y_pos: usize,
    value: u32,
    is_part: bool,
}

struct MappedChar {
    x_pos: usize,
    y_pos: usize,
    character: char,
}

pub fn run() {
    if let Ok(lines) = utils::read_lines("./input/day3") {
        let mut numbers = Vec::<Number>::new();
        let mut mapped_chars = Vec::<MappedChar>::new();
        for (y, line) in lines.enumerate() {
            if let Ok(text) = line {
                let mut skip_until_not_digit = false;
                for (x, char) in text.chars().enumerate() {
                    if !char.is_digit(10) {
                        skip_until_not_digit = false;
                    }

                    if skip_until_not_digit {
                        continue;
                    }

                    if !char_is_period(&char) {
                        mapped_chars.push(MappedChar{x_pos: x, y_pos: y, character: char});
                    }

                    if char.is_digit(10) {
                        skip_until_not_digit = true;
                        if let Some(found_number) = get_number(&text, x){
                            let number = Number {
                                start_x: found_number.1,
                                end_x: found_number.2,
                                y_pos: y,
                                value: found_number.0,
                                is_part: false
                            };
                            println!("{}", number.value);
                        }
                    }
                }
            }
        }
    }
}

fn get_number(text: &String, from: usize) -> Option<(u32, usize, usize)> {
    if let Some(clipped_text) = text.get(from..) {
        for (i, char) in clipped_text.chars().enumerate() {
            if !char.is_digit(10) {
                let num_string = clipped_text.get(0..i);
                if let Some(number) = num_string {
                    if let Ok(parsed_num) = number.parse::<u32>() {
                        return Some((parsed_num, from, from + i))
                    }
                }
            }
        }
    }
    return None;
}

fn char_is_period(character: &char) -> bool {
    match character {
        '.' => true,
        _ => false
    }
}

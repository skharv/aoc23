use super::utils;

struct Location {
    x: i32,
    y: i32,
    gifts: u32,
}

pub fn run() {
   if let Ok(lines) = utils::read_lines("./input/day3") {
        let mut locations = Vec::<Location>::new();
        let mut position: (i32, i32) = (0, 0);
        let origin = Location{x: position.0, y: position.1, gifts: 1};
        locations.push(origin);
        let mut total = 0;

        for line in lines {
            if let Ok(text) = line {
                for char in text.chars() {
                    let location_change = translate_direction(char);
                    position.0 += location_change.0;
                    position.1 += location_change.1;
                    
                    let mut location_exists = false;
                    for location in locations.iter_mut() {
                        if location.x == position.0 && location.y == position.1 {
                            location_exists = true;
                            location.gifts += 1;
                            println!("{},{} now has {} gifts",  location.x, location.y, location.gifts);
                        }
                    }
                    if !location_exists {
                        locations.push(Location{x: position.0, y: position.1, gifts: 1});
                        if let Some(last) = locations.last() {
                            println!("{},{} now has {} gifts",  last.x, last.y, last.gifts);
                        }
                    }
                }
                for location in locations.iter() {
                    if location.gifts >= 1 {
                        total += 1;
                    }
                }
            }
        }
        
        println!("Part 1 Answer: {}", total);
   }
}

fn translate_direction(direction: char) -> (i32, i32) {
    match direction {
        '^' => return (0, 1),
        '>' => return (1, 0),
        '<' => return (-1, 0),
        'v' => return (0, -1),
        _ => {
            println!("ERROR");
            return (0, 0);
        }
    }
}

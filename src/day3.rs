use super::utils;

struct Location {
    x: i32,
    y: i32,
    gifts: u32,
}

pub fn run() {
    if let Ok(lines) = utils::read_lines("./input/day3") {
        let mut locations = Vec::<Location>::new();
        let mut santa_pos: (i32, i32) = (0, 0);
        let mut robo_santa_pos: (i32, i32) = (0, 0);
        let origin = Location{x: santa_pos.0, y: santa_pos.1, gifts: 1};
        locations.push(origin);
        let mut total = 0;

        for line in lines {
            if let Ok(text) = line {
                for (index, char) in text.chars().enumerate() {
                    let location_change = translate_direction(char);
                    if index % 2 != 0 {
                        santa_pos.0 += location_change.0;
                        santa_pos.1 += location_change.1;

                        let mut location_exists = false;
                        for location in locations.iter_mut() {
                            if location.x == santa_pos.0 && location.y == santa_pos.1 {
                                location_exists = true;
                                location.gifts += 1;
                                println!("{},{} now has {} gifts, thanks Santa!",  location.x, location.y, location.gifts);
                            }
                        }
                        if !location_exists {
                            locations.push(Location{x: santa_pos.0, y: santa_pos.1, gifts: 1});
                            if let Some(last) = locations.last() {
                                println!("{},{} now has {} gifts, thanks Santa!",  last.x, last.y, last.gifts);
                            }
                        }
                    } else {
                        robo_santa_pos.0 += location_change.0;
                        robo_santa_pos.1 += location_change.1;

                        let mut location_exists = false;
                        for location in locations.iter_mut() {
                            if location.x == robo_santa_pos.0 && location.y == robo_santa_pos.1 {
                                location_exists = true;
                                location.gifts += 1;
                                println!("{},{} now has {} gifts, thanks Robo Santa!",  location.x, location.y, location.gifts);
                            }
                        }
                        if !location_exists {
                            locations.push(Location{x: robo_santa_pos.0, y: robo_santa_pos.1, gifts: 1});
                            if let Some(last) = locations.last() {
                                println!("{},{} now has {} gifts, thanks Robo Santa!",  last.x, last.y, last.gifts);
                            }
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

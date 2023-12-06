use super::utils;

const RED: u32 = 12;
const GREEN: u32 = 12;
const BLUE: u32 = 14;

struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

pub fn run() {
    if let Ok(lines) = utils::read_lines("./input/day2"){
        let mut games = Vec::<Game>::new();
        let mut total = 0;
        for line in lines {
            if let Ok(text) = line {
            games.push(generate_game(&text));
            }
        }

        for game in games {
            let mut red = false;
            let mut green = false;
            let mut blue = false;
            for handful in game.handfuls {
                if handful.red <= RED {
                    red = true;
                }
                if handful.green <= GREEN {
                    green = true;
                }
                if handful.blue <= BLUE {
                    blue = true;
                }
            }
            if red && green && blue {
                total += game.id;
            }
        }
        println!("{}", total);
    } else {
        println!("file not found");
    }
}

fn generate_game(text: &String) -> Game {
    let game = Game{id: get_game_number(text), handfuls: get_handfuls(text)};
    return game;
}

fn get_game_number(text: &String) -> u32 {
    if let (Some(colon_index), Some(space_index)) = (text.find(":"), text.find(" "), ) {
        if let Some(game_number_text) = text.get(space_index+1..colon_index) {
            if let Ok(game_number) = game_number_text.to_string().parse::<u32>() {
                return game_number;
            }
        }
    }
    return 0;
}

fn get_handfuls(text: &String) -> Vec::<Handful> {
    let mut handfuls = Vec::<Handful>::new();
    let mut indicies = Vec::<usize>::new();

    for (index, char) in text.chars().enumerate() {
        if char == ':' || char == ';' {
            indicies.push(index);
        }
    }

    for (index, value) in indicies.iter().enumerate() {
        if index != indicies.len()-1 {
            if let Some(handful_text) = text.get(value.to_owned()..indicies[index+1].to_owned()) {
                let (r, g, b) = get_handful(&handful_text.to_string());
                let handful = Handful{red: r, green: g, blue: b};
                handfuls.push(handful);
            }
        } else {
            if let Some(handful_text) = text.get(value.to_owned()..text.len()) {
                let (r, g, b) = get_handful(&handful_text.to_string());
                let handful = Handful{red: r, green: g, blue: b};
                handfuls.push(handful);
            }
        }
    }

    return handfuls;
}

fn get_handful(text: &String) -> (u32, u32, u32) {
    return (get_quantity(text, "red"), get_quantity(text, "green"), get_quantity(text, "blue"));
}

fn get_quantity(text: &String, find: &str) -> u32 {
    if let Some(index) = text.find(find) {
        if let Some(text_amount) = text.get(index-3..index) {
            if let Ok(parsed_amount) = text_amount.to_string().trim().parse::<u32>() {
                return parsed_amount;   
            }
        }
    }
    return 0;
}

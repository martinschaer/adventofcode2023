use log::debug;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

use adventofcode2023::run;

#[derive(Debug, Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

type Rgb = Game;

fn tokenize(line: &str) -> (u32, Vec<Game>) {
    let trimmed = line.strip_prefix("Game ").unwrap();
    let (game_id, game_list) = trimmed.split_once(": ").unwrap();
    let games_strs: Vec<&str> = game_list.split("; ").collect();
    let mut games = Vec::new();
    for game_str in games_strs {
        let colors = game_str.split(", ");
        let mut game = Game::default();
        for color in colors {
            let (color_value, color_name) = color.split_once(' ').unwrap();
            match color_name {
                "red" => game.red = color_value.parse::<u32>().unwrap(),
                "green" => game.green = color_value.parse::<u32>().unwrap(),
                "blue" => game.blue = color_value.parse::<u32>().unwrap(),
                _ => panic!("Unknown color: {}", color_name),
            }
        }
        games.push(game);
    }
    (game_id.parse::<u32>().unwrap(), games)
}

fn read_games(file: &str) -> Result<(u32, u32)> {
    let file = File::open(file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let mut sum = 0;
    let mut mins_sum = 0;
    let reds = 12;
    let greens = 13;
    let blues = 14;

    while buf_reader.read_line(&mut contents).unwrap() > 0 {
        // trim_end() removes the newline
        let (game_id, tries) = tokenize(contents.trim_end());
        debug!("game {}: {:?}", game_id, tries);
        let mut is_valid = true;
        let mut mins = Rgb::default();
        for game in tries {
            if game.red > reds || game.green > greens || game.blue > blues {
                is_valid = false;
            }
            mins.red = mins.red.max(game.red);
            mins.green = mins.green.max(game.green);
            mins.blue = mins.blue.max(game.blue);
        }
        if is_valid {
            sum += game_id;
        }
        mins_sum += mins.red * mins.green * mins.blue;
        contents.clear();
    }
    Ok((sum, mins_sum))
}

fn main() {
    // remove timestamp from logs
    env_logger::builder().format_timestamp(None).init();

    // read CLI args
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap_or_else(|| {
        println!("Usage: RUST_LOG=<info|bebug|error> {} <file>", args[0]);
        std::process::exit(1);
    });

    // execute functions
    run(String::from("read_games"), read_games, file_path);

    std::process::exit(0);
}

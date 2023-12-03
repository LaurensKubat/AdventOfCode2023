use std::{
    fs::{read_to_string, File},
    path::Path,
};

use game::{Game, LocalError};

mod game;

fn main() {
    let path = Path::new("input.txt");
    let lines: Vec<String> = read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let g: Result<Vec<Game>, LocalError> = lines.iter().map(|line| Game::parse(line)).collect();
    let games = g.unwrap();
    let total_valid_games = games
        .iter()
        .filter(|g| g.is_valid(max_red, max_green, max_blue))
        .fold(0, |acc, g| {
            g.id + acc
        });
    println!("star 1: {:?}", total_valid_games);


    let min_bag_sizes: Vec<(u32, u32, u32)> = games.iter().map(|g| g.min_bag_size()).collect();
    let sum_of_game_power = min_bag_sizes.iter().fold(0, |acc, (r, g, b)| {
        acc + (r * g * b)
    });

    println!("star 2: {:?}", sum_of_game_power)
}

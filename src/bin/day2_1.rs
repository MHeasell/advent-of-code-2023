use std::{io::BufReader, io::BufRead, fs::File};

use lazy_static::lazy_static;
use regex::Regex;

const TOTAL_RED: i32 = 12;
const TOTAL_GREEN: i32 = 13;
const TOTAL_BLUE: i32 = 14;

fn main() {
    let file = File::open("data/day2/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let games = lines.map(|l| parse(&l.unwrap()));

    let sum = games.filter(|g| is_game_possible(g)).map(|g| g.id).sum::<i32>();

    println!("{}", sum);
}

#[derive(Debug)]
struct Reveal {
    red: i32,
    green: i32,
    blue: i32
}

#[derive(Debug)]
struct GameInfo {
    id: i32,
    reveals: Vec<Reveal>
}

fn is_reveal_possible(r: &Reveal) -> bool {
    r.red <= TOTAL_RED && r.green <= TOTAL_GREEN && r.blue <= TOTAL_BLUE
}

fn is_game_possible(g: &GameInfo) -> bool {
    g.reveals.iter().all(is_reveal_possible)
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    static ref PART_REGEX: Regex = Regex::new(r"^(\d+) (red|green|blue)$").unwrap();
}

fn parse_reveal(reveal_str: &str) -> Reveal {
    let mut reveal = Reveal{
        red: 0,
        green: 0,
        blue: 0
    };
    reveal_str.split(", ").for_each(|part| {
        let captures = PART_REGEX.captures(part).unwrap();
        let num = captures[1].parse::<i32>().unwrap();
        match &captures[2] {
            "red" => reveal.red = num,
            "green" => reveal.green = num,
            "blue" => reveal.blue = num,
            _ => panic!("invalid color")
        };
    });
    reveal
}

fn parse_reveals(reveals_str: &str) -> Vec<Reveal> {
    reveals_str.split("; ").map(parse_reveal).collect()
}

fn parse(line: &str) -> GameInfo {
    let captures = GAME_REGEX.captures(line).unwrap();
    let game_id = captures[1].parse::<i32>().unwrap();
    let game_data = &captures[2];
    let reveals = parse_reveals(game_data);
    GameInfo {
        id: game_id,
        reveals: reveals
    }
}

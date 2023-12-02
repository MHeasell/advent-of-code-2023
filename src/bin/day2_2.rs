use std::{io::BufReader, io::BufRead, fs::File};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day2/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let games = lines.map(|l| parse(&l.unwrap()));

    let sum = games.map(|g| get_power(&get_min_cubes_needed(&g))).sum::<i32>();

    println!("{}", sum);
}

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32
}

#[derive(Debug)]
struct GameInfo {
    _id: i32,
    reveals: Vec<CubeSet>
}

fn elem_max(a: &CubeSet, b: &CubeSet) -> CubeSet {
    CubeSet {
        red: a.red.max(b.red),
        green: a.green.max(b.green),
        blue: a.blue.max(b.blue),
    }
}

fn get_min_cubes_needed(g: &GameInfo) -> CubeSet {
    g.reveals.iter().copied().reduce(|a, b| elem_max(&a, &b)).unwrap()
}

fn get_power(s: &CubeSet) -> i32 {
    s.red * s.green * s.blue
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    static ref PART_REGEX: Regex = Regex::new(r"^(\d+) (red|green|blue)$").unwrap();
}

fn parse_reveal(reveal_str: &str) -> CubeSet {
    let mut reveal = CubeSet{
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

fn parse_reveals(reveals_str: &str) -> Vec<CubeSet> {
    reveals_str.split("; ").map(parse_reveal).collect()
}

fn parse(line: &str) -> GameInfo {
    let captures = GAME_REGEX.captures(line).unwrap();
    let game_id = captures[1].parse::<i32>().unwrap();
    let game_data = &captures[2];
    let reveals = parse_reveals(game_data);
    GameInfo {
        _id: game_id,
        reveals: reveals
    }
}

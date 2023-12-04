use std::{io::BufReader, io::BufRead, fs::File, collections::HashSet};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day4/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let sum = lines.map(|line| get_score(parse_card(&line))).sum::<i32>();

    println!("{}", sum);
}

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"^Card +\d+: ([^|]+) \| (.+)$").unwrap();
}

type Card = (Vec<i32>, Vec<i32>);

fn get_score((winning, have): Card) -> i32 {
    let win_set = winning.iter().collect::<HashSet<_>>();
    let count = have.iter().filter(|num| win_set.contains(num)).count();
    if count == 0 { return 0; }
    2_i32.pow(count as u32 - 1)
}

fn parse_numbers(nums: &str) -> Vec<i32> {
    nums.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn parse_card(line: &str) -> Card {
    let captures = CARD_REGEX.captures(line).unwrap();
    (parse_numbers(&captures[1]), parse_numbers(&captures[2]))
}

use std::{io::BufReader, io::BufRead, fs::File, collections::HashSet};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day4/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let cards = lines.map(|line| {
        parse_card(&line)
    }).collect::<Vec<_>>();

    let mut counts = vec![1; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let num_cards = counts[idx];
        let score = get_score(card);
        let end = (idx+1+score).min(counts.len());
        for i in idx+1..end {
            counts[i] += num_cards;
        }
    }

    let sum = counts.iter().sum::<i32>();

    println!("{}", sum);
}

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"^Card +\d+: ([^|]+) \| (.+)$").unwrap();
}

type Card = (Vec<i32>, Vec<i32>);

fn get_score((winning, have): &Card) -> usize {
    let win_set = winning.iter().collect::<HashSet<_>>();
    let count = have.iter().filter(|num| win_set.contains(num)).count();
    count
}

fn parse_numbers(nums: &str) -> Vec<i32> {
    nums.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn parse_card(line: &str) -> Card {
    let captures = CARD_REGEX.captures(line).unwrap();
    (parse_numbers(&captures[1]), parse_numbers(&captures[2]))
}

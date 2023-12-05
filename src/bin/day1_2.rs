use std::{fs::File, io::BufRead, io::BufReader};

use lazy_static::lazy_static;
use regex::{Match, Regex};

fn main() {
    let file = File::open("data/day1/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let val = lines.map(|l| recover_value(&l.unwrap())).sum::<i32>();

    println!("{}", val);
}

fn word_to_digit(word: &str) -> Option<i32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_match(m: &str) -> i32 {
    word_to_digit(m).unwrap_or_else(|| m.parse().unwrap())
}

lazy_static! {
    static ref DIGIT_REGEX: Regex =
        Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
}

// Last match could overlap a previous match, e.g. "oneight".
// If we naively use Regex.find_iter to iterate all matches
// we will miss the last "eight" because the engine matched
// the "one" and will not find overlapping matches.
// Therefore I wrote this custom thing instead to find the last match.
fn find_last<'a>(r: &Regex, s: &'a str) -> Option<Match<'a>> {
    let m = r.find(s);
    m.map(|m| {
        let idx = m.start();
        let next_match = find_last(r, s.get(idx + 1..).unwrap());
        next_match.unwrap_or(m)
    })
}

fn recover_value(line: &str) -> i32 {
    let first_match = DIGIT_REGEX.find(line).unwrap();
    let last_match = find_last(&DIGIT_REGEX, line).unwrap();

    let first = parse_match(first_match.as_str());
    let last = parse_match(last_match.as_str());

    (10 * first) + last
}

use std::{io::BufReader, io::BufRead, fs::File};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day3/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let sum = lines.iter().enumerate().flat_map(|(idx, line)| {
        let numbers = parse_numbers(line);
        numbers.iter()
        .filter(|n| is_valid_number(if idx > 0 { Some(&lines[idx-1])}  else {None}, &lines[idx], if idx < lines.len() - 1 { Some(&lines[idx+1])}  else { None}, n))
        .map(|n| n.value)
        .collect::<Vec<_>>()
    }).sum::<i32>();

    println!("{}", sum);
}

struct NumInfo {
    start: usize,
    end: usize,
    value: i32
}

fn is_symbol(ch: char) -> bool {
    ch != '.' && !char::is_ascii_digit(&ch)
}

fn is_valid_number(l1: Option<&str>, l2: &str, l3: Option<&str>, n: &NumInfo) -> bool {
    let grid_width = l2.len();

    let start = if n.start > 0 { n.start - 1 } else {n.start};
    let end = if n.end < grid_width { n.end + 1 } else { grid_width };

    if let Some(l1) = l1 {
        for i in start..end {
            if is_symbol(l1.as_bytes()[i] as char) {
                return true;
            }
        }
    }

    if n.start > 0 {
        if is_symbol(l2.as_bytes()[n.start-1] as char) {
            return true;
        }
    }
    if n.end < grid_width {
        if is_symbol(l2.as_bytes()[n.end] as char) {
            return true;
        }
    }

    if let Some(l3) = l3 {
        for i in start..end {
            if is_symbol(l3.as_bytes()[i] as char) {
                return true;
            }
        }
    }

    false
}

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}


fn parse_numbers(line: &str) -> Vec<NumInfo> {
    NUM_REGEX.find_iter(line).map(|m| {
        NumInfo {
            start: m.start(),
            end: m.end(),
            value: m.as_str().parse().unwrap()
        }
    }).collect()

}

use std::{io::BufReader, io::BufRead, fs::File};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day3/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let nums = lines.iter().enumerate().flat_map(|(idx, line)| {
        parse_numbers(idx, line)
    }).collect::<Vec<_>>();

    let sum = lines.iter().enumerate()
    .flat_map(|(idx, line)| find_gears(idx, line))
    .filter_map(|gear| {
            let adjacent_nums = nums.iter().filter(|num| is_adjacent(num, &gear)).collect::<Vec<_>>();
            if adjacent_nums.len() == 2 {
                Some(adjacent_nums[0].value * adjacent_nums[1].value)
            } else { None }
    }).sum::<i32>();

    println!("{}", sum);
}

fn find_gears(idx: usize, line: &str) -> Vec<GearInfo> {
    line.bytes().enumerate().filter_map(|(pos, b)| {
        if b == b'*' { Some(GearInfo{
            idx: pos,
            line: idx
        }) } else { None }
    }).collect()
}

#[derive(Debug)]
struct GearInfo {
    line: usize,
    idx: usize,
}

fn is_adjacent(num: &NumInfo, gear: &GearInfo) -> bool {
    if num.line == gear.line {
        return num.start == gear.idx + 1 || num.end == gear.idx;
    }
    let s = if gear.idx > 0 { gear.idx - 1 } else { 0 };
    let e = gear.idx + 1;

    if num.line == gear.line + 1 || (gear.line > 0 && num.line == gear.line - 1){
        return num.start <= e && num.end-1 >= s
    }

    false
}

#[derive(Debug)]
struct NumInfo {
    line: usize,
    start: usize,
    end: usize,
    value: i32
}

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

fn parse_numbers(line_num: usize, line: &str) -> Vec<NumInfo> {
    NUM_REGEX.find_iter(line).map(|m| {
        NumInfo {
            line: line_num,
            start: m.start(),
            end: m.end(),
            value: m.as_str().parse().unwrap()
        }
    }).collect()
}

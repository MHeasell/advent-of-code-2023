use std::fs::{self};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input_str = fs::read_to_string("data/day15/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    parts: Vec<String>,
}

struct Lens {
    label: String,
    focal_length: usize,
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"^([a-z]+)(=|-)(\d+)?$").unwrap();
}

fn solve(input: &Input) -> i64 {
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for p in &input.parts {
        let captures = INSTRUCTION_REGEX.captures(&p).unwrap();
        let label = &captures[1];
        let op = &captures[2];

        if op == "-" {
            let box_num = hash(label);
            let lens = boxes[box_num]
                .iter()
                .enumerate()
                .find(|(_, l)| l.label == label);
            if let Some((idx, _)) = lens {
                boxes[box_num].remove(idx);
            }
        } else if op == "=" {
            let opand = &captures[3];
            let focal_length = opand.parse::<usize>().unwrap();
            let box_num = hash(label);
            let lens = boxes[box_num]
                .iter()
                .enumerate()
                .find(|(_, l)| l.label == label);
            if let Some((idx, _)) = lens {
                boxes[box_num][idx] = Lens {
                    focal_length,
                    label: label.to_string(),
                }
            } else {
                boxes[box_num].push(Lens {
                    label: label.to_string(),
                    focal_length,
                })
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_num, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(slot, l)| (box_num + 1) * (slot + 1) * (l.focal_length))
        })
        .map(|x| x as i64)
        .sum()
}

fn hash(p: &str) -> usize {
    let mut val = 0_usize;
    for b in p.bytes() {
        val += b as usize;
        val *= 17;
        val = val % 256;
    }
    val
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let parts = lines[0].split(',').map(|x| x.to_string());
    Input {
        parts: parts.collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 145);
    }
}

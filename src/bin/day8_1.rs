use std::{fs::File, io::BufRead, io::BufReader};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day8/input").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let instructions = parse_instructions(&lines.next().unwrap());
    lines.next();

    let nodes = lines.map(|l| parse_node(&l)).collect::<Vec<_>>();

    let result = follow_path(&instructions, &nodes);

    println!("{}", result);
}

fn follow_path(instructions: &[Instruction], nodes: &[Node]) -> i64 {
    let mut idx = 0;

    let mut curr_node: &str = "AAA";

    let mut count = 0;
    while curr_node != "ZZZ" {
        let node_info = nodes.iter().find(|n| n.name == curr_node).unwrap();

        let i = instructions[idx];
        let next = match i {
            Instruction::Left => &node_info.left,
            Instruction::Right => &node_info.right,
        };

        curr_node = next;

        idx += 1;
        if idx >= instructions.len() {
            idx = 0;
        }

        count += 1;
    }

    return count;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

lazy_static! {
    static ref NODE_REGEX: Regex = Regex::new(r"^(\w+) = \((\w+), (\w+)\)").unwrap();
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_node(line: &str) -> Node {
    let captures = NODE_REGEX.captures(line).unwrap();

    Node {
        name: captures[1].to_string(),
        left: captures[2].to_string(),
        right: captures[3].to_string(),
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("invalid instruction"),
        })
        .collect()
}

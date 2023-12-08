use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day8/input").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let instructions = parse_instructions(&lines.next().unwrap());
    lines.next();

    let nodes = lines.map(|l| parse_node(&l)).collect::<Vec<_>>();

    let nodes_lookup = nodes
        .iter()
        .map(|n| (n.name.as_str(), n))
        .collect::<HashMap<_, _>>();

    let start_nodes = nodes
        .iter()
        .map(|n| n.name.as_str())
        .filter(|n| n.ends_with("A"))
        .collect::<Vec<_>>();

    // We're gonna take a leap of faith and assume that we always arrive back
    // at the goal after repeating the full set of instructions some
    // whole number of times, and therefore the answer we're looking for
    // is just the LCM of the steps of each individual ghost.
    let steps = start_nodes
        .iter()
        .map(|n| follow_path_old(&instructions, &nodes_lookup, n))
        .collect::<Vec<_>>();
    let lcm = steps.iter().copied().reduce(lcm).unwrap();

    println!("{}", lcm);
}

fn lcm(a: i64, b: i64) -> i64 {
    let x = gcd(a, b);
    (a * b) / x
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}

fn follow_path_old(
    instructions: &[Instruction],
    nodes_lookup: &HashMap<&str, &Node>,
    start: &str,
) -> i64 {
    let mut idx = 0;

    let mut curr_node = start;

    let mut count = 0;
    while !curr_node.ends_with("Z") {
        let node_info = nodes_lookup.get(curr_node).unwrap();

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

#[derive(Debug, Clone)]
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

use std::{
    collections::HashMap,
    fs::{self},
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input_str = fs::read_to_string("data/day20/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

fn succ(
    states: &mut HashMap<String, ModuleState>,
    modules: &[Module],
    name: &str,
    signal: bool,
    sender_name: &str,
) -> Vec<(String, bool, String)> {
    let s = states.get_mut(name);
    if s.is_none() {
        return vec![];
    }
    let s = s.unwrap();

    let neighbours = &modules.iter().find(|m| m.name == name).unwrap().neighbours;

    let next_signal = match s {
        ModuleState::Normal => Some(signal),
        ModuleState::Conj { last_pulse } => {
            last_pulse.insert(sender_name.to_string(), signal);
            // dbg!(name);
            // dbg!(&last_pulse);
            if last_pulse.values().all(|x| *x) {
                // dbg!("emitted low");
                Some(false)
            } else {
                // dbg!("emitted high");
                Some(true)
            }
        }
        ModuleState::FlipFlop { on } => {
            if signal {
                None
            } else {
                *on = !*on;
                Some(*on)
            }
        }
    };

    if let Some(next_signal) = next_signal {
        return neighbours
            .iter()
            .map(|n| (n.to_string(), next_signal, name.to_string()))
            .collect();
    }

    return vec![];
}

fn push_button(states: &mut HashMap<String, ModuleState>, modules: &[Module]) -> (i64, i64) {
    let mut count_low = 0;

    let mut count_high = 0;

    let mut signals: Vec<(String, bool, String)> = vec![];
    signals.push(("broadcaster".to_string(), false, "button".to_string()));
    count_low += 1;

    while !signals.is_empty() {
        let new_signals = signals
            .iter()
            .flat_map(|(name, signal, sender_name)| {
                succ(states, modules, name, *signal, sender_name)
            })
            .collect::<Vec<_>>();
        count_high += new_signals.iter().filter(|s| s.1).count();
        count_low += new_signals.iter().filter(|s| !s.1).count();
        // dbg!(&new_signals);
        signals = new_signals;
    }

    (count_low as i64, count_high as i64)
}

fn solve(input: &Input) -> i64 {
    solve_inner(input, 1000)
}

fn solve_inner(input: &Input, num_pushes: i64) -> i64 {
    // dbg!(&input.modules);
    let mut states = HashMap::<String, ModuleState>::new();
    for m in &input.modules {
        match m.mod_type.as_str() {
            "&" => {
                let hm = input
                    .modules
                    .iter()
                    .filter(|m2| m2.neighbours.contains(&m.name))
                    .map(|m2| (m2.name.to_string(), false))
                    .collect::<HashMap<_, _>>();
                states.insert(m.name.clone(), ModuleState::Conj { last_pulse: hm })
            }
            "%" => states.insert(m.name.clone(), ModuleState::FlipFlop { on: false }),
            "" => states.insert(m.name.clone(), ModuleState::Normal),
            _ => panic!(),
        };
    }

    // dbg!(&states);

    let mut count_low = 0;
    let mut count_high = 0;

    for _ in 0..num_pushes {
        let (new_low, new_high) = push_button(&mut states, &input.modules);
        count_low += new_low;
        count_high += new_high;
    }

    (count_low * count_high) as i64
}

#[derive(Debug)]
enum ModuleState {
    Normal,
    FlipFlop { on: bool },
    Conj { last_pulse: HashMap<String, bool> },
}

#[derive(Debug)]
struct Input {
    modules: Vec<Module>,
}

lazy_static! {
    static ref THING_REGEX: Regex = Regex::new(r"^([%&]?)([a-z]+) -> (.+)$").unwrap();
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    mod_type: String,
    neighbours: Vec<String>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let modules = lines
        .iter()
        .map(|l| {
            let c = THING_REGEX.captures(l).unwrap();
            Module {
                name: c[2].to_string(),
                mod_type: c[1].to_string(),
                neighbours: c[3].split(", ").map(|l| l.to_string()).collect(),
            }
        })
        .collect::<Vec<_>>();
    Input { modules }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
        let input = parse_input(&input_str);
        let answer = solve_inner(&input, 1);

        assert_eq!(answer, 8 * 4);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        let input = parse_input(&input_str);
        let answer = solve_inner(&input, 1);

        assert_eq!(answer, 4 * 4);
    }
}

use std::fs::{self};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input_str = fs::read_to_string("data/day19/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Condition {
    GreaterThan(i64),
    LessThan(i64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum RuleOutcome {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Rule {
    Outcome(RuleOutcome),
    Condition(ConditionRule),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ConditionRule {
    prop: char,
    condition: Condition,
    outcome: RuleOutcome,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug)]
struct Input {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

fn solve(input: &Input) -> i64 {
    input
        .parts
        .iter()
        .filter(|p| is_accepted(p, &input.workflows))
        .map(|p| get_part_rating(p))
        .sum()
}

fn get_part_rating(p: &Part) -> i64 {
    p.x + p.m + p.a + p.s
}

fn eval_outcome(p: &Part, workflow: &Workflow) -> RuleOutcome {
    for rule in &workflow.rules {
        match rule {
            Rule::Outcome(o) => return o.clone(),
            Rule::Condition(c) => {
                let prop_val = match c.prop {
                    'x' => p.x,
                    'm' => p.m,
                    'a' => p.a,
                    's' => p.s,
                    _ => panic!(),
                };
                let passes = match c.condition {
                    Condition::GreaterThan(x) => prop_val > x,
                    Condition::LessThan(x) => prop_val < x,
                };
                if passes {
                    return c.outcome.clone();
                }
            }
        }
    }

    panic!("impossible?")
}

fn is_accepted(p: &Part, workflows: &[Workflow]) -> bool {
    let mut workflow_name = "in".to_string();

    loop {
        let workflow = workflows.iter().find(|w| w.name == workflow_name).unwrap();
        let outcome = eval_outcome(p, &workflow);
        match outcome {
            RuleOutcome::Accept => {
                return true;
            }
            RuleOutcome::Reject => {
                return false;
            }
            RuleOutcome::Workflow(name) => {
                workflow_name = name.clone();
            }
        }
    }
}

lazy_static! {
    static ref WORKFLOW_REGEX: Regex = Regex::new(r"^([a-z]+)\{([^}]+)\}$").unwrap();
    static ref RULE_REGEX: Regex = Regex::new(r"^([xmas])([<>])(\d+):([a-z]+|A|R)$").unwrap();
    static ref RULE2_REGEX: Regex = Regex::new(r"^([a-z]+|A|R)$").unwrap();
    static ref PART_REGEX: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
}

fn parse_outcome(o: &str) -> Option<RuleOutcome> {
    if let Some(c) = RULE2_REGEX.captures(o) {
        let outcome = match &c[1] {
            "A" => RuleOutcome::Accept,
            "R" => RuleOutcome::Reject,
            _ => RuleOutcome::Workflow(c[1].to_string()),
        };
        return Some(outcome);
    }

    None
}

fn parse_rule(l: &str) -> Rule {
    if let Some(o) = parse_outcome(l) {
        return Rule::Outcome(o);
    }

    let c = RULE_REGEX.captures(l).unwrap();

    Rule::Condition(ConditionRule {
        prop: c[1].chars().next().unwrap(),
        condition: match &c[2] {
            "<" => Condition::LessThan(c[3].parse().unwrap()),
            ">" => Condition::GreaterThan(c[3].parse().unwrap()),
            _ => panic!("impossible"),
        },
        outcome: parse_outcome(&c[4]).unwrap(),
    })
}

fn parse_workflow(l: &str) -> Workflow {
    let caps = WORKFLOW_REGEX.captures(l).unwrap();
    Workflow {
        name: caps[1].to_string(),
        rules: caps[2]
            .split(',')
            .map(|r| parse_rule(r))
            .collect::<Vec<_>>(),
    }
}

fn parse_part(l: &str) -> Part {
    let c = PART_REGEX.captures(l).unwrap();
    Part {
        x: c[1].parse().unwrap(),
        m: c[2].parse().unwrap(),
        a: c[3].parse().unwrap(),
        s: c[4].parse().unwrap(),
    }
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let split = lines.split(|l| l.is_empty()).collect::<Vec<_>>();

    Input {
        workflows: split[0]
            .iter()
            .map(|l| parse_workflow(l))
            .collect::<Vec<_>>(),
        parts: split[1].iter().map(|l| parse_part(l)).collect::<Vec<_>>(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        let input = parse_input(&input_str);

        dbg!(&input);
        let answer = solve(&input);

        assert_eq!(answer, 19114);
    }
}

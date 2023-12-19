use core::fmt;
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
struct TheoryPart {
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

#[derive(Debug)]
struct Input {
    workflows: Vec<Workflow>,
}

fn solve(input: &Input) -> i64 {
    let input_part = TheoryPart {
        x: Interval::new(1, 4000),
        m: Interval::new(1, 4000),
        a: Interval::new(1, 4000),
        s: Interval::new(1, 4000),
    };

    let accepted = get_accepted(&input_part, &input.workflows, "in");
    // dbg!(&accepted);
    assert!(accepted.iter().enumerate().all(|p| {
        accepted
            .iter()
            .enumerate()
            .filter(|x| x.0 != p.0)
            .all(|q| !overlaps(p.1, q.1))
    }));
    accepted.iter().map(|p| count_combos(p)).sum()
}

// Lost all my time here due to typo-ing one of the member names... :(
fn count_combos(p: &TheoryPart) -> i64 {
    p.x.len() * p.m.len() * p.a.len() * p.s.len()
}

fn overlaps(a: &TheoryPart, b: &TheoryPart) -> bool {
    a.x.overlaps(&b.x) && a.m.overlaps(&b.m) && a.a.overlaps(&b.a) && a.s.overlaps(&b.s)
}

fn get_prop(p: &TheoryPart, prop: char) -> &Interval {
    match prop {
        'x' => &p.x,
        'm' => &p.m,
        'a' => &p.a,
        's' => &p.s,
        _ => panic!(),
    }
}

fn with_prop(p: &TheoryPart, prop: char, val: Interval) -> TheoryPart {
    let mut new_p = p.clone();
    match prop {
        'x' => new_p.x = val,
        'm' => new_p.m = val,
        'a' => new_p.a = val,
        's' => new_p.s = val,
        _ => panic!(),
    }
    new_p
}

// left: the part less than val
// right: the part greater than or erqual to val
fn interval_split(i: &Interval, val: i64) -> (Option<Interval>, Option<Interval>) {
    if i.last < val {
        return (Some(*i), None);
    }

    if i.first >= val {
        return (None, Some(*i));
    }

    return (
        Some(Interval::new(i.first, val - 1)),
        Some(Interval::new(val, i.last)),
    );
}

// left: the accepted part
// right: the rejected part
fn interval_gt(i: &Interval, val: i64) -> (Option<Interval>, Option<Interval>) {
    let (less, ge) = interval_split(i, val + 1);
    return (ge, less);
}

// left: the accepted part
// right: the rejected part
fn interval_lt(i: &Interval, val: i64) -> (Option<Interval>, Option<Interval>) {
    let (less, ge) = interval_split(i, val);
    return (less, ge);
}

// returns
// 1. the part that would pass the rule + the outcome
// 2. the part that would fail the rule
fn eval_rule(
    p: &TheoryPart,
    rule: &Rule,
) -> (Option<(TheoryPart, RuleOutcome)>, Option<TheoryPart>) {
    match rule {
        Rule::Outcome(o) => return (Some((*p, o.clone())), None),
        Rule::Condition(c) => {
            let prop_val = get_prop(p, c.prop);

            let (accepted_val, rejected_val) = match c.condition {
                Condition::GreaterThan(x) => interval_gt(prop_val, x),
                Condition::LessThan(x) => interval_lt(prop_val, x),
            };
            let accepted = accepted_val.map(|v| (with_prop(p, c.prop, v), c.outcome.clone()));
            let rejected = rejected_val.map(|v| with_prop(p, c.prop, v));

            return (accepted, rejected);
        }
    }
}

fn eval_outcomes(p: &TheoryPart, workflow: &Workflow) -> Vec<(TheoryPart, RuleOutcome)> {
    let mut output = Vec::new();

    let mut curr_p = *p;
    for rule in &workflow.rules {
        let (passing, failing) = eval_rule(&curr_p, rule);
        if let Some(passing) = passing {
            output.push(passing);
        }
        if let Some(failing) = failing {
            curr_p = failing;
        } else {
            return output;
        }
    }

    panic!("should never get here?")
}

fn get_accepted(p: &TheoryPart, workflows: &[Workflow], workflow_name: &str) -> Vec<TheoryPart> {
    let workflow = workflows.iter().find(|w| w.name == workflow_name).unwrap();
    let result = eval_outcomes(p, &workflow)
        .iter()
        .flat_map(|o| match &o.1 {
            RuleOutcome::Accept => vec![o.0],
            RuleOutcome::Reject => vec![],
            RuleOutcome::Workflow(w) => get_accepted(&o.0, workflows, &w),
        })
        .collect::<Vec<_>>();

    result
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

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let split = lines.split(|l| l.is_empty()).collect::<Vec<_>>();

    Input {
        workflows: split[0]
            .iter()
            .map(|l| parse_workflow(l))
            .collect::<Vec<_>>(),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Interval {
    first: i64,
    last: i64,
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.first, self.last)
    }
}

impl Interval {
    fn new(first: i64, last: i64) -> Interval {
        Interval { first, last }
    }

    fn len(&self) -> i64 {
        self.last - self.first + 1
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.last >= other.first && self.first <= other.last
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

        // dbg!(&input);
        let answer = solve(&input);

        assert_eq!(answer, 167409079868000);
    }

    #[test]
    fn test_eval_outcomes() {
        let workflow_str = "px{a<2006:qkq,m>2090:A,rfg}";
        let workflow = parse_workflow(&workflow_str);

        let input_part = TheoryPart {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        };

        let outcomes = eval_outcomes(&input_part, &workflow);
        assert_eq!(
            outcomes,
            vec![
                (
                    TheoryPart {
                        x: Interval::new(1, 4000),
                        m: Interval::new(1, 4000),
                        a: Interval::new(1, 2005),
                        s: Interval::new(1, 4000),
                    },
                    RuleOutcome::Workflow("qkq".to_string())
                ),
                (
                    TheoryPart {
                        x: Interval::new(1, 4000),
                        m: Interval::new(2091, 4000),
                        a: Interval::new(2006, 4000),
                        s: Interval::new(1, 4000),
                    },
                    RuleOutcome::Accept
                ),
                (
                    TheoryPart {
                        x: Interval::new(1, 4000),
                        m: Interval::new(1, 2090),
                        a: Interval::new(2006, 4000),
                        s: Interval::new(1, 4000),
                    },
                    RuleOutcome::Workflow("rfg".to_string())
                )
            ]
        );
    }

    #[test]
    fn test_eval_outcomes_2() {
        let workflow_str = "rfg{s<537:gd,x>2440:R,A}";
        let workflow = parse_workflow(&workflow_str);

        let input_part = TheoryPart {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        };

        let outcomes = eval_outcomes(&input_part, &workflow);
        assert_eq!(
            outcomes,
            vec![
                (
                    TheoryPart {
                        x: Interval::new(1, 4000),
                        m: Interval::new(1, 4000),
                        a: Interval::new(1, 4000),
                        s: Interval::new(1, 536),
                    },
                    RuleOutcome::Workflow("gd".to_string())
                ),
                (
                    TheoryPart {
                        x: Interval::new(2441, 4000),
                        m: Interval::new(1, 4000),
                        a: Interval::new(1, 4000),
                        s: Interval::new(537, 4000),
                    },
                    RuleOutcome::Reject
                ),
                (
                    TheoryPart {
                        x: Interval::new(1, 2440),
                        m: Interval::new(1, 4000),
                        a: Interval::new(1, 4000),
                        s: Interval::new(537, 4000),
                    },
                    RuleOutcome::Accept
                )
            ]
        );
    }

    #[test]
    fn test_get_accepted_1() {
        let workflow_strs = "rfg{s<537:gd,x>2440:R,A}
gd{A}";
        let workflows = workflow_strs
            .lines()
            .map(|l| parse_workflow(l))
            .collect::<Vec<_>>();

        let input_part = TheoryPart {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        };

        let outcomes = get_accepted(&input_part, &workflows, "rfg");
        assert_eq!(
            outcomes,
            vec![
                TheoryPart {
                    x: Interval::new(1, 4000),
                    m: Interval::new(1, 4000),
                    a: Interval::new(1, 4000),
                    s: Interval::new(1, 536),
                },
                TheoryPart {
                    x: Interval::new(1, 2440),
                    m: Interval::new(1, 4000),
                    a: Interval::new(1, 4000),
                    s: Interval::new(537, 4000),
                }
            ]
        );
    }

    #[test]
    fn test_get_accepted_2() {
        let workflow_strs = "aaaa{s<537:A,R}";
        let workflows = workflow_strs
            .lines()
            .map(|l| parse_workflow(l))
            .collect::<Vec<_>>();

        let input_part = TheoryPart {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        };

        let outcomes = get_accepted(&input_part, &workflows, "aaaa");
        assert_eq!(
            outcomes,
            vec![TheoryPart {
                x: Interval::new(1, 4000),
                m: Interval::new(1, 4000),
                a: Interval::new(1, 4000),
                s: Interval::new(1, 536),
            }]
        );
    }

    #[test]
    fn test_get_accepted_3() {
        let workflow_strs = "aaaa{s<537:R,A}";
        let workflows = workflow_strs
            .lines()
            .map(|l| parse_workflow(l))
            .collect::<Vec<_>>();

        let input_part = TheoryPart {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        };

        let outcomes = get_accepted(&input_part, &workflows, "aaaa");
        assert_eq!(
            outcomes,
            vec![TheoryPart {
                x: Interval::new(1, 4000),
                m: Interval::new(1, 4000),
                a: Interval::new(1, 4000),
                s: Interval::new(537, 4000),
            }]
        );
    }

    #[test]
    fn test_get_accepted_4() {
        let workflow_strs = "aaaa{s<537:R,R}";
        let workflows = workflow_strs
            .lines()
            .map(|l| parse_workflow(l))
            .collect::<Vec<_>>();

        let input_part = TheoryPart {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        };

        let outcomes = get_accepted(&input_part, &workflows, "aaaa");
        assert_eq!(outcomes, vec![]);
    }

    #[test]
    fn test_count_combos() {
        let input_part = TheoryPart {
            x: Interval::new(1, 2),
            m: Interval::new(1, 2),
            a: Interval::new(1, 2),
            s: Interval::new(1, 2),
        };

        let result = count_combos(&input_part);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_interval_gt() {
        assert_eq!(
            interval_gt(&Interval::new(1, 5), 4),
            (Some(Interval::new(5, 5)), Some(Interval::new(1, 4)))
        );

        assert_eq!(
            interval_gt(&Interval::new(5, 8), 4),
            (Some(Interval::new(5, 8)), None)
        );

        assert_eq!(
            interval_gt(&Interval::new(1, 4), 4),
            (None, Some(Interval::new(1, 4)))
        );
    }

    #[test]
    fn test_interval_lt() {
        assert_eq!(
            interval_lt(&Interval::new(1, 5), 4),
            (Some(Interval::new(1, 3)), Some(Interval::new(4, 5)))
        );

        assert_eq!(
            interval_lt(&Interval::new(4, 8), 4),
            (None, Some(Interval::new(4, 8)))
        );

        assert_eq!(
            interval_lt(&Interval::new(1, 3), 4),
            (Some(Interval::new(1, 3)), None)
        );
    }
}

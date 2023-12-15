use std::fs::{self};

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

fn solve(input: &Input) -> i64 {
    input
        .parts
        .iter()
        .map(|p| {
            let mut val = 0_i64;
            for b in p.bytes() {
                val += b as i64;
                val *= 17;
                val = val % 256;
            }
            val
        })
        .sum()
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

        assert_eq!(answer, 1320);
    }
}

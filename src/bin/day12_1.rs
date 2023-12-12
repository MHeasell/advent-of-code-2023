use std::fs::{self};

fn main() {
    let input_str = fs::read_to_string("data/day12/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    lines: Vec<(Vec<char>, Vec<i64>)>,
}

fn count_possibilities(line: &[char], groups: &[i64]) -> i64 {
    if groups.len() == 0 {
        // if any broken, but no more groups, this state is impossible
        if line.iter().any(|c| *c == '#') {
            return 0;
        }

        return 1;
    }
    if line.len() == 0 {
        return 0;
    }

    let group_size = groups[0] as usize;
    if line.len() < group_size {
        return 0;
    }

    if line.len() == group_size {
        if groups.len() == 1 && line.iter().all(|c| *c == '#' || *c == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    let mut count = 0;
    if line.iter().take(group_size).all(|c| *c == '#' || *c == '?')
        && (line[group_size] == '?' || line[group_size] == '.')
    {
        count += count_possibilities(&line[group_size + 1..], &groups[1..]);
    }
    if line[0] == '?' || line[0] == '.' {
        count += count_possibilities(&line[1..], groups);
    }

    count
}

fn solve(input: &Input) -> i64 {
    input
        .lines
        .iter()
        .map(|l| count_possibilities(&l.0, &l.1))
        // .inspect(|x| println!("{:?}", x))
        .sum()
}

fn parse_line(l: &str) -> (Vec<char>, Vec<i64>) {
    let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
    (
        parts[0].chars().collect::<Vec<_>>(),
        parts[1]
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let parsed_lines = lines.iter().map(|l| parse_line(l));
    Input {
        lines: parsed_lines.collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2() {
        let input_str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 21);
    }

    #[test]
    fn test_count() {
        let line = "???.### 1,1,3";
        let l = parse_line(&line);

        let result = count_possibilities(&l.0, &l.1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count2() {
        let line = ".??..??...?##. 1,1,3";
        let l = parse_line(&line);

        let result = count_possibilities(&l.0, &l.1);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count3() {
        {
            let line = "### 1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 0);
        }

        {
            let line = "??? 1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 3);
        }

        {
            let line = "?###???????? 3,2,1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 10);
        }

        {
            let line = "??????? 2,1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 10);
        }

        {
            let line = "???? 1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 4);
        }

        {
            let line = ".???? 1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 4);
        }

        {
            let line = "??? 2,1";
            let l = parse_line(&line);
            let result = count_possibilities(&l.0, &l.1);
            assert_eq!(result, 0);
        }
    }
}

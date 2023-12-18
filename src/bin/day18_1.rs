use std::{
    collections::HashSet,
    fs::{self},
    hash::Hash,
};

fn main() {
    let input_str = fs::read_to_string("data/day18/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    lines: Vec<(char, i64)>,
}

fn to_dir(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!(),
    }
}

fn solve(input: &Input) -> i64 {
    let mut hm = HashSet::<Position>::new();
    let mut p = pos(0, 0);
    hm.insert(p);
    for l in &input.lines {
        let d = to_dir(l.0);
        for _ in 0..l.1 {
            p = p.move_in_direction(d);
            hm.insert(p);
        }
    }

    let min_x = hm.iter().map(|p| p.x).min().unwrap();
    let min_y = hm.iter().map(|p| p.y).min().unwrap();
    let max_x = hm.iter().map(|p| p.x).max().unwrap();
    let max_y = hm.iter().map(|p| p.y).max().unwrap();

    let width = (max_x + 1) - (min_x - 1) + 1;
    let height = (max_y + 1) - (min_y - 1) + 1;
    let area = width * height;

    let outsides = flood_fill(pos(min_x - 1, min_y - 1), |p| {
        DIRECTIONS
            .iter()
            .copied()
            .map(|d| p.move_in_direction(d))
            .filter(|p| {
                p.x >= min_x - 1 && p.y >= min_y - 1 && p.x <= max_x + 1 && p.y <= max_y + 1
            })
            .filter(|p| !hm.contains(p))
            .collect()
    });

    let insides = area - outsides.len() as i64;

    insides
}

fn flood_fill<T, F>(start: T, succ: F) -> HashSet<T>
where
    T: Eq + Hash + Copy,
    F: Fn(&T) -> Vec<T>,
{
    let mut stack = vec![start];
    let mut seen = HashSet::from([start]);

    while let Some(elem) = stack.pop() {
        let neighbours = succ(&elem);
        for n in neighbours {
            if seen.insert(n) {
                stack.push(n);
            }
        }
    }

    seen
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    Input {
        lines: lines
            .iter()
            .map(|l| {
                let split = l.split_ascii_whitespace().collect::<Vec<_>>();
                (
                    split[0].chars().next().unwrap(),
                    split[1].parse::<i64>().unwrap(),
                )
            })
            .collect(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

fn pos(x: i64, y: i64) -> Position {
    Position { x, y }
}

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
//
const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 62);
    }
}

use std::fs::{self};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input_str = fs::read_to_string("data/day18/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    lines: Vec<(Direction, i64)>,
}

fn dig_clockwise(instructions: &[(Direction, i64)]) -> i64 {
    // We assume input starts in the top-left corner and goes clockwise.
    // We'll dig out a big rectangle with the following rules:
    //
    // 1. Whenever we go right we dig the current row further.
    // 2. Whenever we go down we dig out the entire row,
    //    i.e. current position + everything to the left.
    // 3. Whenever we go left we don't need to do anything,
    //    we're just returning through already dug squares.
    // 4. Whenever we go up we'll "un-dig" everything to the left of us.
    //
    // We don't need to worry about crossing X=0 or Y=0.
    // If that happens, everything will work out the same.
    // We'll dig or undig a negative number of squares
    // and it will all cancel out by the time we've finished the loop.

    let mut curr_x = 0;
    let mut acc = 1;

    for l in instructions {
        match l.0 {
            Direction::Right => {
                curr_x += l.1;
                acc += l.1;
            }
            Direction::Left => {
                curr_x -= l.1;
            }
            Direction::Down => {
                acc += (curr_x + 1) * l.1;
            }
            Direction::Up => {
                acc -= curr_x * l.1;
            }
        }
    }

    acc
}

// It took me an embarrassingly long time to come up with this solution.
// But now that I've found it, it's very tidy.
//
// Before I found this, my previous idea was to group up the lines into distinct vertical runs
// (a sequence of lines that go only down and across, or only up and across),
// and then scan row-by-row to count up the intervals that lie between each vertical run.
// However this was extremely fiddly and I never managed to shake out
// all the bugs. In particular, there should always be an even number of vertical runs
// intersecting each row, but I had a *lot* of problems where my code would detect an odd number of runs,
// which is invalid.
//
// Ultimately only after basically giving up in frustration with that approach
// did I cotton onto this much nicer (simpler to implement, more efficient) solution.
// I did still find it very fiddly. In particular I faffed about a lot with worrying what happens
// when the shape crosses the X=0 or Y=0 lines, before I eventually realised that none of that
// was necessary and everything works out.
//
// It's funny that this comment (and, by extension, my anguish) is far longer than the solution
// at this point. lol.
fn solve(input: &Input) -> i64 {
    // We need the input to be a clockwise dig.
    // We don't know if the input is clockwise or anti-clockwise.
    // If the input is anticlockwise the result will be too small,
    // so we'll just try it both ways and take the larger.
    let answer1 = dig_clockwise(&input.lines);
    let anti_clockwise_input = input
        .lines
        .iter()
        .rev()
        .map(|l| (l.0.reverse(), l.1))
        .collect::<Vec<_>>();
    let answer2 = dig_clockwise(&anti_clockwise_input);
    return answer1.max(answer2);
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^[RUDL] \d+ \(#([a-zA-Z0-9]{5})([a-zA-Z0-9])\)$").unwrap();
}

fn to_dir(c: char) -> Direction {
    match c {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!(),
    }
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    Input {
        lines: lines
            .iter()
            .map(|l| {
                let captures = LINE_REGEX.captures(l).unwrap();
                (
                    to_dir(captures[2].chars().next().unwrap()),
                    i64::from_str_radix(&captures[1], 16).unwrap(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate_cw(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn reflect_around_y(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Down,
        }
    }

    fn reflect_around_x(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }

    fn parse_input_old(s: &str) -> Input {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        Input {
            lines: lines
                .iter()
                .map(|l| {
                    let split = l.split_ascii_whitespace().collect::<Vec<_>>();
                    (
                        to_dir_old(split[0].chars().next().unwrap()),
                        split[1].parse::<i64>().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn to_dir_old(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }

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

        assert_eq!(answer, 952408144115);
    }

    #[test]
    fn test_solve1_old() {
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
        let input = parse_input_old(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 62);
    }

    #[test]
    fn test_solve_case() {
        let input = Input {
            lines: vec![
                (Direction::Right, 4),
                (Direction::Down, 4),
                (Direction::Left, 4),
                (Direction::Up, 4),
            ],
        };
        let answer = solve(&input);

        assert_eq!(answer, 25);

        for stretched in stretch_input(&input.lines) {
            let answer2 = solve(&Input { lines: stretched });
            assert_eq!(answer2, 25);
        }
    }

    #[test]
    fn test_solve_case2() {
        let input = Input {
            lines: vec![
                (Direction::Left, 4),
                (Direction::Up, 4),
                (Direction::Right, 4),
                (Direction::Down, 4),
            ],
        };
        let answer = solve(&input);

        assert_eq!(answer, 25);

        for stretched in stretch_input(&input.lines) {
            let answer2 = solve(&Input { lines: stretched });
            assert_eq!(answer2, 25);
        }
    }

    #[test]
    fn test_solve_case3() {
        let input = Input {
            lines: vec![
                (Direction::Right, 6),
                (Direction::Down, 4),
                (Direction::Left, 2),
                (Direction::Up, 2),
                (Direction::Left, 2),
                (Direction::Down, 2),
                (Direction::Left, 2),
                (Direction::Up, 4),
            ],
        };
        let answer = solve(&input);

        assert_eq!(answer, 33);

        for stretched in stretch_input(&input.lines) {
            let answer2 = solve(&Input { lines: stretched });
            assert_eq!(answer2, 33);
        }
    }

    /// Generates a bunch of variations of the input
    /// that are expected to yield the same answer.
    fn stretch_input(input: &[(Direction, i64)]) -> Vec<Vec<(Direction, i64)>> {
        vec![
            input
                .iter()
                .map(|line| (rotate_cw(line.0), line.1))
                .collect(),
            input
                .iter()
                .map(|line| (rotate_cw(rotate_cw(line.0)), line.1))
                .collect(),
            input
                .iter()
                .map(|line| (rotate_cw(rotate_cw(rotate_cw(line.0))), line.1))
                .collect(),
            input
                .iter()
                .map(|line| (reflect_around_x(line.0), line.1))
                .collect(),
            input
                .iter()
                .map(|line| (reflect_around_y(line.0), line.1))
                .collect(),
            input
                .iter()
                .rev()
                .map(|line| (line.0.reverse(), line.1))
                .collect(),
        ]
    }

    #[test]
    fn test_solve_case4() {
        let input = Input {
            lines: vec![
                (Direction::Right, 6),
                (Direction::Down, 4),
                (Direction::Left, 2),
                (Direction::Up, 2),
                (Direction::Left, 2),
                (Direction::Down, 2),
                (Direction::Left, 2),
                (Direction::Up, 4),
            ],
        };
        let answer = solve(&input);

        assert_eq!(answer, 33);

        for stretched in stretch_input(&input.lines) {
            let answer2 = solve(&Input { lines: stretched });
            assert_eq!(answer2, 33);
        }
    }

    #[test]
    fn test_solve_case5() {
        let input = Input {
            lines: vec![
                (Direction::Right, 1),
                (Direction::Down, 1),
                (Direction::Left, 1),
                (Direction::Up, 1),
            ],
        };
        let expected_answer = 4;

        let answer = solve(&input);
        assert_eq!(answer, expected_answer);

        for stretched in stretch_input(&input.lines) {
            let answer2 = solve(&Input { lines: stretched });
            assert_eq!(answer2, expected_answer);
        }
    }

    #[test]
    fn test_solve_case6() {
        let input = Input {
            lines: vec![
                (Direction::Right, 1),
                (Direction::Up, 1),
                (Direction::Left, 1),
                (Direction::Down, 1),
            ],
        };
        let expected_answer = 4;

        let answer = solve(&input);
        assert_eq!(answer, expected_answer);
    }
}

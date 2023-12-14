use std::fs::{self};

fn main() {
    let input_str = fs::read_to_string("data/day14/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    g: Grid<char>,
}

fn solve(input: &Input) -> i64 {
    (0..input.g.width)
        .map(|c| get_col_load(&input.g, c) as i64)
        .sum()
}

fn fall_rock(col: &mut [char], idx: usize) {
    if idx == 0 {
        return;
    }
    let prev = col[idx - 1];
    let curr = col[idx];
    if prev == '.' {
        col[idx - 1] = curr;
        col[idx] = '.';
        fall_rock(col, idx - 1);
    }
}

fn shift_col(col: &mut [char]) {
    for i in 1..col.len() {
        if col[i] == 'O' {
            fall_rock(col, i);
        }
    }
}

fn get_col_load(g: &Grid<char>, c: usize) -> usize {
    let mut col = (0..g.height()).map(|r| *g.get(c, r)).collect::<Vec<_>>();
    shift_col(&mut col);

    col.iter()
        .enumerate()
        .filter(|(_, c)| **c == 'O')
        .map(|(i, _)| col.len() - i)
        .sum()
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    Input {
        g: Grid::from_strings(&lines),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid<T: Clone> {
    width: usize,
    vec: Vec<T>,
}

impl Grid<char> {
    fn from_strings(lines: &[String]) -> Self {
        if lines.len() == 0 {
            return Grid {
                width: 0,
                vec: vec![],
            };
        }

        let width = lines[0].len();
        let vec = lines.iter().flat_map(|l| l.chars()).collect();
        Grid { width, vec }
    }
}

impl<T: Clone> Grid<T> {
    fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    fn to_vec_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height() {
            Some((y * self.width) + x)
        } else {
            None
        }
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[self.to_vec_index(x, y).unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....

";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 136);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
.
O
.
.
.
#
O
.
.
O
";
        let input = parse_input(&input_str);

        let answer = solve(&input);

        assert_eq!(answer, 17);
    }
}

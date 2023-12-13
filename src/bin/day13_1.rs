use std::fs::{self};

fn main() {
    let input_str = fs::read_to_string("data/day13/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    grids: Vec<Grid<char>>,
}

fn solve(input: &Input) -> i64 {
    input
        .grids
        .iter()
        .map(|g| {
            let col = get_refl_col(g).unwrap_or(0);
            let row = get_refl_row(g).map(|i| i * 100).unwrap_or(0);
            col + row
        })
        .sum()
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let grid_inputs = lines
        .split(|l| l.is_empty())
        .map(|ls| Grid::from_strings(ls));

    Input {
        grids: grid_inputs.collect(),
    }
}

fn is_refl_col(g: &Grid<char>, col: i64) -> bool {
    if col == 0 || col == g.width as i64 {
        return false;
    }

    for i in (0..g.width).map(|i| i as i64) {
        let left = col - 1 - i;
        let right = col + i;
        if left < 0 || right >= g.width as i64 {
            return true;
        }
        if !(0..g.height()).all(|y| g.get(left as usize, y) == g.get(right as usize, y)) {
            return false;
        }
    }
    return true;
}

fn is_refl_row(g: &Grid<char>, row: i64) -> bool {
    if row == 0 || row == g.height() as i64 {
        return false;
    }

    for i in (0..g.height()).map(|i| i as i64) {
        let up = row - 1 - i;
        let down = row + i;
        if up < 0 || down >= g.height() as i64 {
            return true;
        }
        if !(0..g.width).all(|x| g.get(x, up as usize) == g.get(x, down as usize)) {
            return false;
        }
    }
    return true;
}
fn get_refl_col(g: &Grid<char>) -> Option<i64> {
    (0..g.width)
        .find(|i| is_refl_col(g, *i as i64))
        .map(|i| i as i64)
}

fn get_refl_row(g: &Grid<char>) -> Option<i64> {
    (0..g.height())
        .find(|i| is_refl_row(g, *i as i64))
        .map(|i| i as i64)
}

#[derive(Debug, Clone)]
struct Grid<T> {
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

impl<T> Grid<T> {
    fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[self.to_vec_index(x, y).unwrap()]
    }

    fn to_vec_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height() {
            Some((y * self.width) + x)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 405);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 5);
    }

    #[test]
    fn test_solve3() {
        let input_str = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 400);
    }
}

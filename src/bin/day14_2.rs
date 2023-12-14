use std::{
    fs::{self},
    iter::successors,
};

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
    let mut g = input.g.clone();
    run_all_cycles(&mut g);
    get_all_load(&g) as i64
}

fn fall_rock_dir(g: &mut Grid<char>, p: Position, d: Direction) {
    let fall_pos = p.move_in_direction(d);

    let prev = g.try_get_pos(&fall_pos);
    if let Some(prev) = prev {
        let curr = g.get_pos(&p);
        if *prev == '.' {
            g.set_pos(&fall_pos, *curr);
            g.set_pos(&p, '.');
            fall_rock_dir(g, fall_pos, d);
        }
    }
}

fn shift_col(g: &mut Grid<char>, d: Direction) {
    match d {
        Direction::Up => {
            for c in 0..g.width {
                for r in 0..g.height() {
                    if *g.get(c, r) == 'O' {
                        fall_rock_dir(g, pos(c as i64, r as i64), d);
                    }
                }
            }
        }
        Direction::Down => {
            for c in 0..g.width {
                for r in (0..g.height()).rev() {
                    if *g.get(c, r) == 'O' {
                        fall_rock_dir(g, pos(c as i64, r as i64), d);
                    }
                }
            }
        }
        Direction::Left => {
            for r in 0..g.height() {
                for c in 0..g.width {
                    if *g.get(c, r) == 'O' {
                        fall_rock_dir(g, pos(c as i64, r as i64), d);
                    }
                }
            }
        }
        Direction::Right => {
            for r in 0..g.height() {
                for c in (0..g.width).rev() {
                    if *g.get(c, r) == 'O' {
                        fall_rock_dir(g, pos(c as i64, r as i64), d);
                    }
                }
            }
        }
    }
}

fn run_cycle(g: &mut Grid<char>) {
    for d in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ] {
        shift_col(g, d);
    }
}

fn get_all_load(g: &Grid<char>) -> usize {
    g.enumerate()
        .filter(|(_, c)| **c == 'O')
        .map(|(p, _)| g.height() - p.y as usize)
        .sum()
}

fn run_all_cycles(g: &mut Grid<char>) {
    let num_cycles = 1000000000;

    let it = successors(Some(g.clone()), |g| {
        let mut g2 = g.clone();
        run_cycle(&mut g2);
        Some(g2)
    });

    let (preamble, period) = detect_loop(&it).unwrap();
    dbg!(preamble);
    dbg!(period);

    assert!(preamble <= num_cycles);
    for _ in 0..preamble {
        run_cycle(g);
    }

    let remaining_cycles = num_cycles - preamble;

    let remainder = remaining_cycles % period;

    for _ in 0..remainder {
        run_cycle(g);
    }
}

/// Returns (steps before loop, loop length).
///
/// Steps before loop is an overestimation.
/// It is always some multiple of the loop length.
/// After taking that many steps you are guaranteed
/// to be inside the loop, but it doesn't tell you
/// exactly where the loop starts.
fn detect_loop<T, A>(it: &A) -> Option<(usize, usize)>
where
    T: Eq,
    A: Iterator<Item = T> + Clone,
{
    let mut a = it.clone();
    let mut b = it.clone();

    let mut tortoise = a.next()?;
    b.next()?;
    let mut hare = b.next()?;

    let mut steps = 1;

    while tortoise != hare {
        tortoise = a.next()?;
        b.next()?;
        hare = b.next()?;
        steps += 1;
    }

    let loop_length = a.take_while(|x| *x != hare).count() + 1;

    Some((steps, loop_length))
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

    fn signed_to_vec_index(&self, x: i64, y: i64) -> Option<usize> {
        let x2 = usize::try_from(x).ok()?;
        let y2 = usize::try_from(y).ok()?;
        self.to_vec_index(x2, y2)
    }

    fn pos_to_vec_index(&self, pos: &Position) -> Option<usize> {
        self.signed_to_vec_index(pos.x, pos.y)
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[self.to_vec_index(x, y).unwrap()]
    }

    fn get_pos(&self, pos: &Position) -> &T {
        self.get(pos.x.try_into().unwrap(), pos.y.try_into().unwrap())
    }

    fn try_get_pos(&self, pos: &Position) -> Option<&T> {
        self.pos_to_vec_index(pos).map(|i| &self.vec[i])
    }

    fn set_pos(&mut self, pos: &Position, val: T) {
        let index = self.pos_to_vec_index(pos).unwrap();
        self.vec[index] = val;
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    fn pos_iter(&self) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height();
        (0..height).flat_map(move |y| {
            (0..width).map(move |x| Position {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            })
        })
    }

    fn enumerate(&self) -> impl Iterator<Item = (Position, &T)> {
        self.pos_iter().zip(self.iter())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
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

fn pos(x: i64, y: i64) -> Position {
    Position { x, y }
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

        assert_eq!(answer, 64);
    }

    fn solve_old(input: &Input) -> i64 {
        let mut g = input.g.clone();
        shift_col(&mut g, Direction::Up);
        get_all_load(&g) as i64
    }

    #[test]
    fn test_solve_old() {
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
        let answer = solve_old(&input);

        assert_eq!(answer, 136);
    }

    #[test]
    fn test_detect_loop() {
        let it = (0..10).cycle();
        let answer = detect_loop(&it);
        assert_eq!(answer, Some((10, 10)));
    }

    #[test]
    fn test_detect_loop_2() {
        let it = (0..108).chain((0..10).cycle());
        let answer = detect_loop(&it);
        assert_eq!(answer, Some((110, 10)));
    }

    #[test]
    fn test_detect_loop_3() {
        let it = (30..40).chain((0..10).cycle());
        let answer = detect_loop(&it);
        assert_eq!(answer, Some((20, 10)));
    }

    #[test]
    fn test_detect_loop_4() {
        let it = (30..33).chain((0..10).cycle());
        let answer = detect_loop(&it);
        assert_eq!(answer, Some((10, 10)));
    }

    #[test]
    fn test_detect_loop_5() {
        let it = (30..48).chain((0..10).cycle());
        let answer = detect_loop(&it);
        assert_eq!(answer, Some((20, 10)));
    }
}

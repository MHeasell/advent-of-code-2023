use std::{
    collections::HashSet,
    fs::{self},
    hash::Hash,
};

fn main() {
    let input_str = fs::read_to_string("data/day16/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);
    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    g: Grid<char>,
}

fn reflect(d: Direction, val: char) -> Direction {
    match (d, val) {
        (Direction::Right, '/') => Direction::Up,
        (Direction::Left, '/') => Direction::Down,
        (Direction::Up, '/') => Direction::Right,
        (Direction::Down, '/') => Direction::Left,

        (Direction::Right, '\\') => Direction::Down,
        (Direction::Left, '\\') => Direction::Up,
        (Direction::Up, '\\') => Direction::Left,
        (Direction::Down, '\\') => Direction::Right,
        _ => panic!("unhandled"),
    }
}

fn succ_dirs(val: &char, d: Direction) -> Vec<Direction> {
    match (val, d) {
        ('.', _) => vec![d],
        ('/', _) | ('\\', _) => vec![reflect(d, *val)],
        ('-', d) if d == Direction::Left || d == Direction::Right => vec![d],
        ('|', d) if d == Direction::Up || d == Direction::Down => vec![d],
        ('-', _) => vec![Direction::Left, Direction::Right],
        ('|', _) => vec![Direction::Up, Direction::Down],
        _ => panic!("unhandled"),
    }
}

fn succ(g: &Grid<char>, p: Position, d: Direction) -> Vec<(Position, Direction)> {
    let val = g.get_pos(&p);
    let next_dirs = succ_dirs(val, d);

    next_dirs
        .iter()
        .filter_map(|next_d| {
            let new_p = p.move_in_direction(*next_d);
            g.try_get_pos(&new_p).is_some().then_some((new_p, *next_d))
        })
        .collect()
}

fn solve(input: &Input) -> i64 {
    let start = pos(0, 0);

    let seen = flood_fill((start, Direction::Right), |(p, d)| succ(&input.g, *p, *d));
    let energized = seen.iter().map(|x| x.0).collect::<HashSet<_>>();

    // for r in input.g.rows() {
    //     for c in input.g.cols() {
    //         if energized.contains(&pos(c as i64, r as i64)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    energized.len() as i64
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
        g: Grid::from_strings(&lines),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 46);
    }

    #[test]
    fn test_succ() {
        let input_str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
        let input = parse_input(&input_str);
        assert_eq!(
            succ(&input.g, pos(5, 0), Direction::Right),
            vec![(pos(5, 1), Direction::Down)]
        );

        assert_eq!(
            succ(&input.g, pos(4, 1), Direction::Down),
            vec![(pos(5, 1), Direction::Right)]
        );
    }
}

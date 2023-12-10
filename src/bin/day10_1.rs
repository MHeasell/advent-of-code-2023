use std::{fmt::Debug, fs::File, io::BufRead, io::BufReader, iter::successors};

fn main() {
    let file = File::open("data/day10/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let grid = parse_grid(&lines);
    let start = grid.find_pos(|x| *x == TerrainType::Start).unwrap();

    let loop_length = DIRECTIONS
        .iter()
        .find_map(|d| get_loop_length(&grid, start, *d))
        .unwrap();

    let mut answer = loop_length / 2;
    if loop_length % 2 != 0 {
        answer += 1;
    }

    println!("{}", answer);
}

fn get_loop_length(g: &Grid<TerrainType>, start: Position, d: Direction) -> Option<i64> {
    successors(Some((start, d)), |(pos, exit_dir)| {
        let next_pos = pos.move_in_direction(*exit_dir);
        let next_terrain_type = g.try_get_pos(&next_pos)?;
        let next_exit_dir = get_exit_dir(*next_terrain_type, exit_dir.reverse())?;
        Some((next_pos, next_exit_dir))
    })
    .enumerate()
    .skip(1)
    .find_map(|(i, (pos, _))| (pos == start).then_some(i as i64))
}

fn get_exit_dir(terrain_type: TerrainType, entry_dir: Direction) -> Option<Direction> {
    // If we're not allowed to enter from the entry dir,
    // then there can be no exit dir.
    if !get_exits(terrain_type).contains(&entry_dir) {
        return None;
    }

    let exits = get_exits(terrain_type)
        .iter()
        .copied()
        .filter(|x| *x != entry_dir)
        .collect::<Vec<_>>();
    match exits.len() {
        0 => None,
        1 => Some(exits[0]),
        // This can only be the starting square, just return some garbage,
        // it doesn't matter to our caller
        _ => {
            assert_eq!(terrain_type, TerrainType::Start);
            Some(Direction::Up)
        }
    }
}

fn get_exits(t: TerrainType) -> &'static [Direction] {
    match t {
        TerrainType::VerticalPipe => &[Direction::Up, Direction::Down],
        TerrainType::HorizontalPipe => &[Direction::Left, Direction::Right],
        TerrainType::DownToLeftPipe => &[Direction::Down, Direction::Left],
        TerrainType::DownToRightPipe => &[Direction::Down, Direction::Right],
        TerrainType::UpToLeftPipe => &[Direction::Up, Direction::Left],
        TerrainType::UpToRightPipe => &[Direction::Up, Direction::Right],
        TerrainType::Ground => &[],
        TerrainType::Start => &DIRECTIONS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerrainType {
    Ground,
    VerticalPipe,
    HorizontalPipe,
    UpToLeftPipe,
    UpToRightPipe,
    DownToLeftPipe,
    DownToRightPipe,
    Start,
}

fn c_to_t(c: char) -> TerrainType {
    match c {
        '|' => TerrainType::VerticalPipe,
        '-' => TerrainType::HorizontalPipe,
        'L' => TerrainType::UpToRightPipe,
        'J' => TerrainType::UpToLeftPipe,
        '7' => TerrainType::DownToLeftPipe,
        'F' => TerrainType::DownToRightPipe,
        '.' => TerrainType::Ground,
        'S' => TerrainType::Start,
        _ => panic!("invalid terrain symbol"),
    }
}

fn parse_grid(lines: &[String]) -> Grid<TerrainType> {
    let mut g = Grid::new(lines[0].len(), lines.len(), TerrainType::Ground);
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            g.set(j, i, c_to_t(c));
        }
    }
    g
}

#[derive(Debug)]
struct Grid<T: Clone> {
    width: usize,
    vec: Vec<T>,
}
impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, val: T) -> Self {
        Self {
            width,
            vec: vec![val; width * height],
        }
    }

    pub fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    fn to_vec_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height() {
            Some((y * self.width) + x)
        } else {
            None
        }
    }

    fn to_pos(&self, vec_index: usize) -> Option<Position> {
        if vec_index < self.vec.len() {
            let x = vec_index % self.width;
            let y = vec_index / self.width;
            Some(Position {
                x: x as i64,
                y: y as i64,
            })
        } else {
            None
        }
    }

    fn try_get_pos(&self, pos: &Position) -> Option<&T> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        self.to_vec_index(pos.x as usize, pos.y as usize)
            .map(|i| &self.vec[i])
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        let index = self.to_vec_index(x, y).unwrap();
        self.vec[index] = val;
    }

    fn find_pos<F: Fn(&T) -> bool>(&self, pred: F) -> Option<Position> {
        self.vec.iter().enumerate().find_map(|(i, e)| {
            if pred(e) {
                Some(self.to_pos(i).unwrap())
            } else {
                None
            }
        })
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

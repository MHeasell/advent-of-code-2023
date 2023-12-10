use std::{
    collections::HashSet, fmt::Debug, fs::File, hash::Hash, io::BufRead, io::BufReader,
    iter::successors,
};

fn main() {
    let file = File::open("data/day10/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let grid = parse_grid(&lines);

    let answer = solve(&grid);

    println!("{}", answer);
}

fn solve(grid: &Grid<TerrainType>) -> usize {
    let loop_nodes = get_loop_coords(grid);

    // print_set(grid, &loop_nodes);

    let insides = get_inside_nodes(&grid, &loop_nodes);

    // println!();

    // print_set(grid, &insides);

    insides.len()
}

fn get_inside_nodes(g: &Grid<TerrainType>, loop_nodes: &HashSet<Position>) -> HashSet<Position> {
    let start = Position { x: 0, y: 0 };

    let seen = fill(start, |p| {
        DIRECTIONS
            .iter()
            .copied()
            .filter(|d| can_move(g, loop_nodes, *d, *p))
            .map(|d| p.move_in_direction(d))
            .filter(|p| p.x >= 0 && p.x <= g.width as i64 && p.y >= 0 && p.y <= g.height() as i64)
            .collect()
    });

    // print_set(g, &seen);

    g.pos_iter()
        .filter(|pos| !seen.contains(pos) && !loop_nodes.contains(pos))
        .collect()
}

fn fill<T, F>(start: T, succ: F) -> HashSet<T>
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

fn can_move(
    g: &Grid<TerrainType>,
    loop_items: &HashSet<Position>,
    d: Direction,
    p: Position,
) -> bool {
    let (side1_pos, side2_pos) = match d {
        Direction::Right => (p.move_in_direction(Direction::Up), p),
        Direction::Down => (p, p.move_in_direction(Direction::Left)),
        Direction::Left => (
            p.move_in_direction(Direction::Left),
            p.move_in_direction(Direction::Left)
                .move_in_direction(Direction::Up),
        ),
        Direction::Up => (
            p.move_in_direction(Direction::Up)
                .move_in_direction(Direction::Left),
            p.move_in_direction(Direction::Up),
        ),
    };

    let side1_val = if loop_items.contains(&side1_pos) {
        g.try_get_pos(&side1_pos).unwrap_or(&TerrainType::Ground)
    } else {
        &TerrainType::Ground
    };
    let side2_val = if loop_items.contains(&side2_pos) {
        g.try_get_pos(&side2_pos).unwrap_or(&TerrainType::Ground)
    } else {
        &TerrainType::Ground
    };
    !is_blocked(*side1_val, *side2_val, d)
}

fn is_blocked(left: TerrainType, right: TerrainType, d: Direction) -> bool {
    let left_dir = d.rotate_ccw();
    let right_dir = d.rotate_cw();
    get_exits(left).contains(&right_dir) && get_exits(right).contains(&left_dir)
}

fn get_loop_coords(g: &Grid<TerrainType>) -> HashSet<Position> {
    let start = g.find_pos(|x| *x == TerrainType::Start).unwrap();
    DIRECTIONS
        .iter()
        .find_map(|d| get_loop_coords_inner(&g, start, *d))
        .unwrap()
        .into_iter()
        .collect::<HashSet<_>>()
}

fn get_loop_coords_inner(
    g: &Grid<TerrainType>,
    start: Position,
    d: Direction,
) -> Option<Vec<Position>> {
    let nodes_iter = successors(Some((start, d)), |(pos, exit_dir)| {
        let next_pos = pos.move_in_direction(*exit_dir);
        let next_terrain_type = g.try_get_pos(&next_pos)?;
        let next_exit_dir = get_exit_dir(*next_terrain_type, exit_dir.reverse())?;
        Some((next_pos, next_exit_dir))
    });

    let mut out = vec![start];

    for (pos, _) in nodes_iter.skip(1) {
        if pos == start {
            return Some(out);
        }
        out.push(pos);
    }

    None
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

fn t_to_c(t: TerrainType) -> char {
    match t {
        TerrainType::VerticalPipe => '|',
        TerrainType::HorizontalPipe => '-',
        TerrainType::UpToRightPipe => 'L',
        TerrainType::UpToLeftPipe => 'J',
        TerrainType::DownToLeftPipe => '7',
        TerrainType::DownToRightPipe => 'F',
        TerrainType::Ground => '.',
        TerrainType::Start => 'S',
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

#[allow(dead_code)]
fn print_set(g: &Grid<TerrainType>, elems: &HashSet<Position>) {
    for y in 0..g.height() {
        for x in 0..g.width {
            if elems.contains(&Position {
                x: x as i64,
                y: y as i64,
            }) {
                print!("{}", t_to_c(*g.get(x, y)));
            } else {
                print!("o");
            }
        }
        print!("\n");
    }
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

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[self.to_vec_index(x, y).unwrap()]
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
    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_blocked() {
        assert!(!is_blocked(c_to_t('7'), c_to_t('F'), Direction::Up));
        assert!(!is_blocked(c_to_t('F'), c_to_t('7'), Direction::Down));
        assert!(!is_blocked(c_to_t('|'), c_to_t('|'), Direction::Up));
        assert!(!is_blocked(c_to_t('|'), c_to_t('|'), Direction::Down));
        assert!(!is_blocked(c_to_t('-'), c_to_t('-'), Direction::Left));
        assert!(!is_blocked(c_to_t('-'), c_to_t('-'), Direction::Right));
    }

    #[test]
    fn test_can_move() {
        {
            let h = HashSet::<Position>::from([Position { x: 0, y: 0 }, Position { x: 1, y: 0 }]);
            let g = parse_grid(&["7F"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
            assert!(can_move(&g, &h, Direction::Down, Position { x: 1, y: 0 }));
            assert!(can_move(&g, &h, Direction::Up, Position { x: 1, y: 1 }));
        }

        {
            let h = HashSet::<Position>::from([Position { x: 0, y: 0 }, Position { x: 1, y: 0 }]);
            let g = parse_grid(&["F7"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
            assert!(!can_move(&g, &h, Direction::Down, Position { x: 1, y: 0 }));
            assert!(!can_move(&g, &h, Direction::Up, Position { x: 1, y: 1 }));
        }

        {
            let h = HashSet::<Position>::from([Position { x: 0, y: 0 }, Position { x: 1, y: 0 }]);
            let g = parse_grid(&["F|"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
            assert!(can_move(&g, &h, Direction::Down, Position { x: 1, y: 0 }));
            assert!(can_move(&g, &h, Direction::Up, Position { x: 1, y: 1 }));
        }

        {
            let s = "\
....
.F7.
.LJ.
....
";
            let h = HashSet::<Position>::from([
                Position { x: 1, y: 1 },
                Position { x: 2, y: 1 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 2 },
            ]);
            let g = parse_grid(&s.lines().map(|x| x.to_string()).collect::<Vec<_>>());
            assert!(!can_move(&g, &h, Direction::Down, Position { x: 2, y: 2 }));
            assert!(!can_move(&g, &h, Direction::Left, Position { x: 2, y: 2 }));
            assert!(!can_move(&g, &h, Direction::Up, Position { x: 2, y: 2 }));
            assert!(!can_move(&g, &h, Direction::Right, Position { x: 2, y: 2 }));

            assert!(!can_move(&g, &h, Direction::Down, Position { x: 2, y: 1 }));
            assert!(!can_move(&g, &h, Direction::Right, Position { x: 1, y: 2 }));
            assert!(!can_move(&g, &h, Direction::Left, Position { x: 3, y: 2 }));
            assert!(!can_move(&g, &h, Direction::Up, Position { x: 2, y: 3 }));
        }
    }

    #[test]
    fn test_get_loop_coords() {
        let s = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let c_set = get_loop_coords(&g);

        let p_set = g
            .pos_iter()
            .zip(g.vec.iter())
            .filter_map(|(p, v)| (*v != TerrainType::Ground).then_some(p))
            .collect::<HashSet<_>>();

        assert_eq!(c_set.len(), p_set.len());
        for elem in c_set {
            assert!(p_set.contains(&elem));
        }
    }

    #[test]
    fn test_get_exit_dir() {
        assert_eq!(
            get_exit_dir(TerrainType::UpToRightPipe, Direction::Down),
            None
        );
        assert_eq!(
            get_exit_dir(TerrainType::UpToRightPipe, Direction::Up),
            Some(Direction::Right)
        );
    }

    #[test]
    fn test_solve1() {
        let s = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_solve1b() {
        let s = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_solve2() {
        let s = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 8);
    }

    #[test]
    fn test_solve3() {
        let s = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 10);
    }

    #[test]
    fn test_solve_custom1() {
        let s = "\
.....
.S7..
.LJ..
.....
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_solve_custom2() {
        let s = "\
S---7
|...|
|...|
L---J
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_solve_custom3() {
        let s = "\
F7F7
||S|
|LJ|
|LJ|
L--J
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_solve_custom4() {
        let s = "\
S7
LJ
F7
LJ
";
        let g = parse_grid(&s.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let answer = solve(&g);
        assert_eq!(answer, 0);
    }
}

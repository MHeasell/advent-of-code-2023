use std::{
    collections::{HashSet, VecDeque},
    fs::{self},
    hash::Hash,
};

fn main() {
    let input_str = fs::read_to_string("data/day17/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("{}", answer);
}

#[derive(Debug)]
struct Input {
    g: Grid<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    p: Position,
    facing_dir: Direction,
    moves_in_dir_taken: i64,
}

// This new successor function makes the search slow.
// It could be optimized by skipping over the intermediate states
// when turning (must travel at least 4 in a straight line afterwards),
// but compiling in release mode made the program run fast enough
// so I didn't bother.
fn succ(g: &Grid<i64>, s: &State) -> Vec<(State, i64)> {
    let succs = DIRECTIONS
        .iter()
        .filter_map(|d| {
            // min 4 straight
            if s.moves_in_dir_taken < 4 {
                if *d != s.facing_dir {
                    // forced to go straight still
                    return None;
                }
                return Some(State {
                    p: s.p.move_in_direction(*d),
                    facing_dir: s.facing_dir,
                    moves_in_dir_taken: s.moves_in_dir_taken + 1,
                });
            }

            // max 10 straight
            if s.moves_in_dir_taken == 10 {
                if *d == s.facing_dir {
                    return None;
                }
            }

            // can't reverse
            if d.reverse() == s.facing_dir {
                return None;
            }

            // turning
            Some(State {
                p: s.p.move_in_direction(*d),
                facing_dir: *d,
                moves_in_dir_taken: if s.facing_dir == *d {
                    s.moves_in_dir_taken + 1
                } else {
                    1
                },
            })
        })
        .filter_map(|ns| g.try_get_pos(&ns.p).map(|cost| (ns, *cost)))
        .collect::<Vec<_>>();

    succs
}

fn solve(input: &Input) -> i64 {
    let goal = pos((input.g.width - 1) as i64, (input.g.height() - 1) as i64);

    dijkstra_search(
        &vec![
            State {
                p: pos(0, 0),
                facing_dir: Direction::Down,
                moves_in_dir_taken: 0,
            },
            State {
                p: pos(0, 0),
                facing_dir: Direction::Right,
                moves_in_dir_taken: 0,
            },
        ],
        |s| succ(&input.g, s),
        |p| p.p == goal && p.moves_in_dir_taken >= 4,
    )
    .unwrap()
}

fn dijkstra_search<T, Succ, GPred>(start: &[T], get_successors: Succ, is_goal: GPred) -> Option<i64>
where
    T: Hash + Eq + Copy,
    Succ: Fn(&T) -> Vec<(T, i64)>,
    GPred: Fn(&T) -> bool,
{
    // Open list could be a min heap instead but who has time to write that lol
    let mut open_list = VecDeque::<(T, i64)>::new();
    let mut closed_set = HashSet::<T>::new();

    for s in start {
        open_list.push_back((*s, 0));
    }

    while let Some((value, cost)) = open_list.pop_front() {
        if is_goal(&value) {
            return Some(cost);
        }

        closed_set.insert(value);

        for (successor_val, successor_cost) in get_successors(&value) {
            if closed_set.contains(&successor_val) {
                continue;
            }

            insert_open_list(&mut open_list, successor_val, cost + successor_cost);
        }
    }

    None
}

fn insert_open_list<T: Eq>(list: &mut VecDeque<(T, i64)>, value: T, priority: i64) {
    let existing_elem = list.iter().enumerate().find(|(_, elem)| elem.0 == value);

    match existing_elem {
        Some((_, elem)) if elem.1 <= priority => {
            return;
        }
        Some((i, _)) => {
            list.remove(i);
        }
        None => {}
    };

    let insert_index = list
        .iter()
        .enumerate()
        .find_map(|(i, elem)| (elem.1 > priority).then_some(i));

    list.insert(insert_index.unwrap_or(list.len()), (value, priority));
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    let rows = lines
        .iter()
        .map(|r| {
            r.chars()
                .map(|n| n.to_string().parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<_>>();
    Input {
        g: Grid::from_vecs(&rows),
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

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid<T> {
    width: usize,
    vec: Vec<T>,
}
impl<T: Clone> Grid<T> {
    fn from_vecs(lines: &[Vec<T>]) -> Self {
        if lines.len() == 0 {
            return Grid {
                width: 0,
                vec: vec![],
            };
        }

        let width = lines[0].len();
        let vec = lines.concat();
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

    fn try_get_pos(&self, pos: &Position) -> Option<&T> {
        self.pos_to_vec_index(pos).map(|i| &self.vec[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 94);
    }

    #[test]
    fn test_solve2() {
        let input_str = "\
111111111111
999999999991
999999999991
999999999991
999999999991
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 71);
    }
}

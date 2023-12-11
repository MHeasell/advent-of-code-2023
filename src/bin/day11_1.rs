use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("data/day11/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let grid = parse_grid(&lines);

    let galaxies = grid
        .vec
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (*v == '#').then_some(grid.to_pos(i).unwrap()))
        .collect::<Vec<_>>();

    let (empty_rows, empty_cols) = get_empties(&grid);
    let expanded_gals = translate_galaxies(&galaxies, &empty_rows, &empty_cols);

    let sum = expanded_gals
        .iter()
        .flat_map(|g| expanded_gals.iter().map(|g2| g2.manhattan_distance(g)))
        .sum::<u64>()
        / 2;

    println!("{}", sum);
}

fn parse_grid(lines: &[String]) -> Grid<char> {
    let mut g = Grid::new(lines[0].len(), lines.len(), '.');
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            g.set(j, i, c);
        }
    }

    g
}

fn get_empties(g: &Grid<char>) -> (Vec<i64>, Vec<i64>) {
    let empty_rows = (0..g.height())
        .filter(|row| (0..g.width).all(|col| *g.get(col, *row) == '.'))
        .map(|x| x as i64)
        .collect();
    let empty_cols = (0..g.width)
        .filter(|col| (0..g.height()).all(|row| *g.get(*col, row) == '.'))
        .map(|x| x as i64)
        .collect();

    (empty_rows, empty_cols)
}

fn translate_galaxies(gals: &[Position], empty_rows: &[i64], empty_cols: &[i64]) -> Vec<Position> {
    gals.iter()
        .map(|p| {
            let offset_x = empty_cols.iter().filter(|r| **r < p.x).count();
            let offset_y = empty_rows.iter().filter(|r| **r < p.y).count();
            Position {
                x: p.x + offset_x as i64,
                y: p.y + offset_y as i64,
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn manhattan_distance(&self, other: &Position) -> u64 {
        let delta_x = other.x.abs_diff(self.x);
        let delta_y = other.y.abs_diff(self.y);
        delta_x + delta_y
    }
}

#[derive(Debug)]
struct Grid<T: Clone> {
    width: usize,
    vec: Vec<T>,
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, val: T) -> Self {
        Self {
            width,
            vec: vec![val; width * height],
        }
    }

    fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[self.to_vec_index(x, y).unwrap()]
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        let index = self.to_vec_index(x, y).unwrap();
        self.vec[index] = val;
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
                x: i64::try_from(x).unwrap(),
                y: i64::try_from(y).unwrap(),
            })
        } else {
            None
        }
    }
}

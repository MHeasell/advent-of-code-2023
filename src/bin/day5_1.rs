use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("data/day5/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let seeds = parse_seeds(&lines[0]);

    let maps = lines[2..]
        .split(|l| l == "")
        .map(|c| parse_chunk(c))
        .collect::<Vec<_>>();

    let answer = seeds
        .iter()
        .map(|s| seed_to_location(*s, &maps))
        .min()
        .unwrap();

    println!("{}", answer);
}

struct Lookup {
    src: Interval,
    dst_start: i64,
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    first: i64,
    last: i64,
}
impl Interval {
    fn from_start_len(start: i64, len: i64) -> Interval {
        Interval {
            first: start,
            last: start + len - 1,
        }
    }

    fn contains(&self, val: i64) -> bool {
        self.first <= val && self.last >= val
    }
}

fn translate_single(num: i64, table: &Lookup) -> Option<i64> {
    if !table.src.contains(num) {
        return None;
    }

    let diff = num - table.src.first;
    Some(table.dst_start + diff)
}

fn translate(num: i64, table: &Vec<Lookup>) -> i64 {
    table
        .iter()
        .find_map(|l| translate_single(num, l))
        .unwrap_or(num)
}

fn seed_to_location(s: i64, maps: &Vec<Vec<Lookup>>) -> i64 {
    maps.iter().fold(s, |s, m| translate(s, m))
}

fn parse_seeds(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_chunk(c: &[String]) -> Vec<Lookup> {
    let vals = &c[1..];
    vals.iter()
        .map(|v| {
            v.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|ns| {
            let a = ns[0];
            let b = ns[1];
            let c = ns[2];
            Lookup {
                src: Interval::from_start_len(b, c),
                dst_start: a,
            }
        })
        .collect::<Vec<_>>()
}

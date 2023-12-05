use std::{io::BufReader, io::BufRead, fs::File};

fn main() {
    let file = File::open("data/day5/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let seeds = parse_seeds(&lines[0]);

    let maps = lines[2..].split(|l| l == "").map(|c| parse_chunk(c)).collect::<Vec<_>>();

    let answer = seeds.iter().flat_map(|s| seed_range_to_location_range(s, &maps)).map(|i| i.first).min().unwrap();

    println!("{}", answer);
}

#[derive(Debug)]
struct Lookup {
    src: Interval,
    dst_start: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval {
    first: i64,
    last: i64,
}
impl Interval {
    fn new(first: i64, last: i64) -> Interval {
        Interval{first, last}
    }
    fn from_start_len(start: i64, len: i64) -> Interval {
        Interval { first: start, last: start + len - 1 }
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.last >= other.first && self.first <= other.last
    }

    fn offset(&self, val: i64) -> Interval {
        Interval { first: self.first + val, last: self.last + val }
    }
}

fn slice_interval(a: &Interval, b: &Interval) -> (Option<Interval>, Option<Interval>, Option<Interval>) {
    if !a.overlaps(b) {
        if a.last < b.first {
            return (Some(*a), None, None);
        }
        return (None, None, Some(*a));
    }

    if a.first < b.first {
        let before = Interval::new(a.first,b.first-1);

        if a.last <= b.last {
            let middle = Interval::new(b.first,a.last);
            (Some(before), Some(middle), None)
        } else {
            let middle = Interval::new(b.first, b.last);
            let after = Interval::new(b.last+1, a.last);
            (Some(before), Some(middle), Some(after))
        }
    } else {
        if a.last <= b.last {
            let middle = Interval::new(a.first, a.last);
            (None, Some(middle), None)
        } else {
            let middle = Interval::new(a.first, b.last);
            let after = Interval::new(b.last+1, a.last);
            (None, Some(middle), Some(after))
        }
    }
}

fn translate_single(i: &Interval, table: &Lookup) -> (Option<Interval>, Option<Interval>, Option<Interval>) {
    let (a, b, c) = slice_interval(i, &table.src);

    if let Some(b) = b {
        let offset = table.dst_start - table.src.first;
        let mapped = b.offset(offset);
        return (a, Some(mapped), c);
    }

    (a, b, c)
}

fn translate(num: &Vec<Interval>, table: &Vec<Lookup>) -> Vec<Interval> {
    let mut v = Vec::new();

    let mut remaining_intervals = num.clone();

    for lookup in table {
        let mut new_remaining = Vec::new();
        for interval in remaining_intervals {
            let (a, b, c) =  translate_single(&interval, lookup);
            if let Some(b) = b {
                v.push(b);
            }
            if let Some(a) = a {
                new_remaining.push(a);
            }
            if let Some(c) = c {
                new_remaining.push(c);
            }
        }
        remaining_intervals = new_remaining;
    }

    for i in remaining_intervals {
        v.push(i);
    }

    v
}

fn seed_range_to_location_range(s: &Interval, maps: &Vec<Vec<Lookup>>) -> Vec<Interval> {
    maps.iter().fold(vec![*s], |s, m| {
        translate(&s, m)
    })
}

fn parse_seeds(line: &str) -> Vec<Interval> {
    line.split_ascii_whitespace().skip(1).collect::<Vec<_>>().chunks(2).map(|x| {
        let na = x[0].parse::<i64>().unwrap();
        let nb = x[1].parse::<i64>().unwrap();
        Interval::from_start_len(na, nb)
    }).collect()
}

fn parse_chunk(c: &[String]) -> Vec<Lookup> {
    let vals = &c[1..];
    vals.iter().map(|v| v.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>()).map(|ns| {
        let a = ns[0];
        let b = ns[1];
        let c = ns[2];
        Lookup {
            src: Interval::from_start_len(b, c),
            dst_start: a
        }
    }).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let lookup = Lookup {
            src: Interval::new(3, 6),
            dst_start: 13,
        };

        let result = translate(&vec![Interval::new(2, 5)], &vec![lookup]);
        assert_eq!(result, vec![Interval::new(13, 15), Interval::new(2, 2)]);
    }

    #[test]
    fn test_translate_2() {
        let interval = Interval {
            first: 79,
            last: 92,
        };

        let lookup_table = vec![
            Lookup {
                src: Interval {
                    first: 98,
                    last: 99,
                },
                dst_start: 50,
            },
            Lookup {
                src: Interval {
                    first: 50,
                    last: 97,
                },
                dst_start: 52,
            },
        ];

        let result = translate(&vec![interval], &lookup_table);

        assert_eq!(result, vec![Interval::new(81, 94)]);
    }
}

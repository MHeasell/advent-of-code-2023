use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("data/day6/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let races = parse_races(&lines);

    let answer = races
        .iter()
        .map(|thing| count_ways_to_beat(&thing))
        .product::<usize>();

    println!("{}", answer);
}

fn count_ways_to_beat(thing: &Race) -> usize {
    (0..thing.time)
        .map(|x| {
            let remaining_time = thing.time - x;
            let distance = remaining_time * x;
            distance
        })
        .filter(|x| *x > thing.distance)
        .count()
}

struct Race {
    time: i64,
    distance: i64,
}

fn parse_races(lines: &[String]) -> Vec<Race> {
    let times = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let distances = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    times
        .iter()
        .enumerate()
        .map(|(i, n)| Race {
            time: *n,
            distance: distances[i],
        })
        .collect()
}

use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("data/day6/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let race = parse_race(&lines);

    let answer = count_ways_to_beat(&race);

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

fn parse_race(lines: &[String]) -> Race {
    let time = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    Race { time, distance }
}

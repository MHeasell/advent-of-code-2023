use std::{fs::File, io::BufRead, io::BufReader, iter::successors};

fn main() {
    let file = File::open("data/day9/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let result = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|t| find_next_val(&t))
        .sum::<i64>();

    println!("{}", result);
}

fn gen_deltas(line: &[i64]) -> Vec<i64> {
    line.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>()
}

fn gen_all_deltas(line: &[i64]) -> Vec<Vec<i64>> {
    successors(Some(Vec::from(line)), |prev| {
        if prev.iter().copied().all(|x| x == 0) {
            None
        } else {
            Some(gen_deltas(prev))
        }
    })
    .collect::<Vec<_>>()
}

fn find_next_val(line: &[i64]) -> i64 {
    gen_all_deltas(line)
        .iter()
        .rfold(0, |acc, x| x.first().unwrap() - acc)
}

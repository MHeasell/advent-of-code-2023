use std::{io::BufReader, io::BufRead, fs::File};

fn main() {
    let file = File::open("data/day1/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let val = lines.map(|l| recover_value(&l.unwrap())).sum::<i32>();

    println!("{}", val);

}

fn recover_value(line: &str) -> i32 {
    let first_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let last_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

    let bytes = line.as_bytes();
    let num_str = format!("{}{}", (bytes[first_idx] as char), (bytes[last_idx] as char));

    num_str.parse().unwrap()
}
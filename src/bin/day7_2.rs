use std::{cmp::Ordering, collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("data/day7/input").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut hands = lines.map(|line| parse_hand(&line)).collect::<Vec<_>>();
    hands.sort_by(|a, b| {
        get_hand_type(&a.cards)
            .cmp(&get_hand_type(&b.cards))
            .reverse()
            .then_with(|| cmp_hands(&a.cards, &b.cards))
    });
    let total = hands
        .iter()
        .enumerate()
        .map(|(i, x)| x.bid * (i as i64 + 1))
        .sum::<i64>();

    println!("{}", total);
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i64,
}

fn cmp_hands(a: &str, b: &str) -> Ordering {
    a.chars()
        .zip(b.chars())
        .map(|(a, b)| c_to_num(&a).cmp(&c_to_num(&b)))
        .find(|c| !c.is_eq())
        .unwrap_or(Ordering::Equal)
}

fn c_to_num(c: &char) -> i64 {
    match c {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("bad card"),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_type_int(counts: &HashMap<char, i64>) -> HandType {
    if let Some(_) = counts.iter().find(|x| *x.1 >= 5) {
        return HandType::Five;
    }

    if let Some(_) = counts.iter().find(|x| *x.1 >= 4) {
        return HandType::Four;
    }

    if let Some(e) = counts.iter().find(|x| *x.1 >= 3) {
        if counts.iter().find(|x| *x.1 >= 2 && x.0 != e.0).is_some() {
            return HandType::FullHouse;
        }
        return HandType::Three;
    }

    if let Some(e) = counts.iter().find(|x| *x.1 >= 2) {
        if counts.iter().find(|x| *x.1 >= 2 && x.0 != e.0).is_some() {
            return HandType::TwoPair;
        }
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn get_hand_type(hand: &str) -> HandType {
    let mut counts: HashMap<char, i64> = HashMap::new();

    for c in hand.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }

    let jokers = *counts.get(&'J').unwrap_or(&0);

    counts.remove(&'J');

    ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']
        .into_iter()
        .map(|c| {
            let mut counts2 = counts.clone();
            counts2.insert(c, counts2.get(&c).unwrap_or(&0) + jokers);
            get_hand_type_int(&counts2)
        })
        .min()
        .unwrap()
}

fn parse_hand(line: &str) -> Hand {
    let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
    Hand {
        cards: parts[0].to_string(),
        bid: parts[1].parse().unwrap(),
    }
}

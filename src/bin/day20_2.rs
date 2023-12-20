fn main() {
    // Very early on, I munged the graph nodes into a .dot file and put it into graphviz.
    // From there it was clear that there were four distinct groups of gates
    // whose results were ANDed together at the end.
    // I resisted trying to actually look at how these worked for a long time
    // and tried thinking about all sorts of other stupid stuff like:
    //
    // 1. can you work out the number by trying to run the system backwards, somehow
    // 2. can I just make the forward simulation faster so that I can brute force it
    //    (I counted the number of flip-flops in the graph and concluded
    //     that the number of states is way too high for this to be practical
    //     and the answer is probably somewhere in the hundreds of billions,
    //     but I tried it anyway. You can get the brute force algo going pretty fast
    //     with some bit twiddling, lol.)
    //
    // Anyway, I finally decided to draw them out more clearly on paper
    // and it was pretty obvious that they were counters
    // that reset once they get to the target value.
    // Would have spotted that a lot sooner if I weren't so stubborn.
    // So I decoded the number being counted to by each counter
    // and that's what you see here.
    // We need all the counters to be at their target
    // and they reset after getting there so the answer is just the LCM
    // of the targets.
    let answer = [3889, 3877, 3803, 3917]
        .into_iter()
        .reduce(|acc, x| lcm(acc, x))
        .unwrap();

    println!("{}", answer);
}

/// Least common multiple of a and b.
pub fn lcm(a: i64, b: i64) -> i64 {
    let x = gcd(a, b);
    (a * b) / x
}

/// Greatest common divisor of a and b.
pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}

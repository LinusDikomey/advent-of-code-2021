pub fn run(input: &str) {
    println!("Day 7:");
    let input = input.trim().split(',').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<_>>();
    // ---------- part 1 ----------
    println!("\tPart 1: {}", optimize(&input, std::convert::identity));
    // ---------- part 2 ----------
    println!("\tPart 2: {}", optimize(&input, |x| (1..=x).sum()));
}

fn optimize(i: &[i32], c: impl Fn(i32) -> i32) -> i32 {
    (*i.iter().min().unwrap()..*i.iter().max().unwrap()).map(|t| i.iter().map(|v| c((v-t).abs())).sum()).min().unwrap()
}

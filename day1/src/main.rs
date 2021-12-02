fn main() {
    // Part 1
    let input = include_str!("input.txt").lines().map(|line| line.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let count = input.iter().zip(input.iter().skip(1)).filter(|(prev, next)| prev < next).count();
    println!("Part 1: {}", count);
    // Part 2
    let count = window(&input).iter().zip(window(&input).iter().skip(1)).filter(|(prev, next)| prev < next).count();
    println!("Part 2: {}", count);

}

fn window(v: &Vec<u32>) -> Vec<u32> {
    v.iter()
        .zip(v.iter().skip(1))
        .zip(v.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect()
}
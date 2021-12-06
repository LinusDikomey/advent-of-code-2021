pub fn run(input: &str) {
    println!("Day 3:");
    // ---------- part 1 ----------
    
    let line_count = input.lines().count() as u32;
    let one_counts = input.lines()
        .map(|line| line.chars().map(|c| if c == '1' { 1 } else { 0 }).collect::<Vec<_>>())
        .reduce(|a, b| a.iter().zip(b.iter()).map(|(a, b)| a + b).collect())
        .unwrap();

    let gamma = one_counts.iter()
        .enumerate()
        .map(|(i, one_count)| if *one_count > line_count - *one_count { 1 << i } else { 0 })
        .reduce(|a, b| a | b)
        .unwrap();
    
    let epsilon = one_counts.iter()
        .enumerate()
        .map(|(i, one_count)| if *one_count < line_count - *one_count { 1 << i } else { 0 })
        .reduce(|a, b| a | b)
        .unwrap();

    println!("\tPart 1: {}", gamma * epsilon);
    // ---------- part 2 ----------

    let oxygen_generator_rating = find_criterion(input, |zeroes, ones| if ones >= zeroes { '1' } else { '0' });
    let co2_scrubber_rating =     find_criterion(input, |zeroes, ones| if zeroes <= ones { '0' } else { '1' });

    println!("\tPart 2: {}", oxygen_generator_rating * co2_scrubber_rating);
}

fn find_criterion<F>(input: &str, f: F) -> i32
where F: Fn(usize, usize) -> char { // f takes zeroes and ones and returns char to filter
    let mut current = input.lines().collect::<Vec<_>>();
    for bit in 0..current[0].len() {
        let ones = current.iter()
        .filter(|line| line.chars().nth(bit).unwrap() == '1')
        .count();
        let zeroes = current.len() - ones;
        let filter = f(zeroes, ones);
        current.retain(|line| line.chars().nth(bit).unwrap() == filter);
        if current.len() == 1 { break; }
    }
    assert_eq!(current.len(), 1);
    i32::from_str_radix(current[0], 2).unwrap()
}
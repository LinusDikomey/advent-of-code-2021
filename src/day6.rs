pub fn run(input: &str) {
    println!("Day 6:");

    let fishes = input.split(',').map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // ---------- part 1 ----------
    println!("\tPart 1: {}", simulate(fishes.clone(), 80));
    // ---------- part 2 ----------
    println!("\tPart 2: {}", simulate_optimized(&fishes, 256));
}


fn simulate(mut fishes: Vec<u32>, days: usize) -> usize {
    for _ in 0..days {
        let mut new = 0;
        for fish in &mut fishes {
            if *fish == 0 {
                new += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        fishes.extend((0..new).map(|_| 8));
    }
    fishes.len()
}

fn simulate_optimized(fishes: &[u32], days: usize) -> u64 {
    let mut fishes_by_day = [0_u64; 9];
    for fish in fishes {
        fishes_by_day[*fish as usize] += 1;
    }
    for _ in 0..days {
        fishes_by_day.rotate_left(1);
        fishes_by_day[6] += fishes_by_day[8];
    }
    fishes_by_day.iter().sum()
}

pub fn run(input: &str) {
    let mut grid = [[0; 10]; 10];
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().map(|c| c as u8 - b'0').enumerate() {
            grid[y][x] = v;
        }
    }
    println!("Day 11:");
    // ---------- part 1 ----------
    let mut grid_p1 = grid.clone();
    println!("\tPart 1: {}", (0..100).map(|_| update(&mut grid_p1).0).sum::<u64>());
    // ---------- part 2 ----------
    let mut step = 1;
    loop {
        if update(&mut grid).1.iter().flatten().find(|e| !*e).is_none() {
            break;
        }
        step += 1;

    } 
    println!("\tPart 2: {}", step);
}

fn update(grid: &mut [[u8; 10]; 10]) -> (u64, [[bool; 10]; 10]) {
    let mut flashed = [[false; 10]; 10];
    let mut flashes = grid.iter_mut()
        .enumerate()
        .map(|(y, row)| 
            row.iter_mut()
                .enumerate()
                .filter_map(move |(x, v)| { *v += 1; if *v > 9 { Some((x, y)) } else { None } }))
        .flatten()
        .collect::<Vec<_>>();

    let mut flash_count = 0;
    while !flashes.is_empty() {
        flash_count += flashes.len() as u64;
        for (x, y) in flashes {
            for ny in (if y == 0 {0} else {y-1})..=(if y == 9 {9} else{y+1}) {
                for nx in (if x == 0 {0} else {x-1})..=(if x == 9 {9} else{x+1}) {
                    if grid[ny][nx] > 0 {
                        grid[ny][nx] += 1;
                    }
                }
            }
            grid[y][x] = 0;
            flashed[y][x] = true;
        }
        flashes = grid.iter_mut()
            .enumerate()
            .map(|(y, row)|
                row.iter_mut()
                    .enumerate()
                    .filter_map(move |(x, v)| if *v > 9 { Some((x, y)) } else { None })
            )
            .flatten()
            .collect::<Vec<_>>();
    }
    (flash_count, flashed)
}
pub fn run(input: &str) {
    let width = input.lines().next().unwrap().len();
    let heightmap: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    let height = heightmap.len();
    let low_points = (0..height).map(|y| {
        let heightmap = &heightmap;
        (0..width).filter_map(move |x| {
            let v = heightmap[y][x];
            let low_point = 
                if x > 0 { v < heightmap[y][x-1] } else { true } &&
                if y > 0 { v < heightmap[y-1][x] } else { true } &&
                if x < width-1  { v < heightmap[y][x+1] } else { true } &&
                if y < height-1 { v < heightmap[y+1][x] } else { true };
            if low_point {
                Some((x,y, v as u64))
            } else {
                None
            }
        })
    }).flatten().collect::<Vec<_>>();

    println!("Day 9:");
    // ---------- part 1 ----------
    println!("\tPart 1: {}", low_points.iter().map(|(_, _, d)| d+1).sum::<u64>());
    // ---------- part 2 ----------

    fn basin_size(x: usize, y: usize, h: &Vec<Vec<u8>>, f: &mut Vec<Vec<bool>>) -> u64 {
        if h[y][x] == 9 { return 0; }
        f[y][x] = true;
        1 + 
            if x > 0 && !f[y][x-1] { basin_size(x-1, y, h, f) } else { 0 } +
            if y > 0 && !f[y-1][x] { basin_size(x, y-1, h, f) } else { 0 } +
            if x < h[0].len()-1 && !f[y][x+1] { basin_size(x+1, y, h, f) } else { 0 } +
            if y < h.len()-1    && !f[y+1][x] { basin_size(x, y+1, h, f) } else { 0 }
    }
    
    let mut basin_sizes = low_points.iter().map(|(x, y, _)| {
        let mut filled = (0..height).map(|_| vec![false; width]).collect();
        basin_size(*x, *y, &heightmap, &mut filled)
    }).collect::<Vec<_>>();
    basin_sizes.sort_unstable();

    println!("\tPart 2: {}", basin_sizes.iter().rev().take(3).product::<u64>());
}
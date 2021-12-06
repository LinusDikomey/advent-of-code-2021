use std::cmp::{min, max};

pub fn run(input: &str) {
    let lines = input.lines()
        .map(|line| {
            let mut points = line.split_terminator(" -> ")
                .map(|coords| {
                    let mut coords = coords.split(',')
                        .map(|num| num.parse::<i64>().unwrap());
                    (coords.next().unwrap(), coords.next().unwrap())
                });
            (points.next().unwrap(), points.next().unwrap())
        })
        .collect::<Vec<_>>();

    println!("Day 5:");
    // ---------- part 1 ----------
    
    let mut grid = [[0_u32; 1000]; 1000];
    for ((x1, y1), (x2, y2)) in &lines {
        if x1 == x2 {
            for y in min(*y1, *y2)..=max(*y1, *y2) {
                //grid[(*x1 * y_size + y) as usize] += 1;
                grid[*x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            for x in min(*x1, *x2)..=max(*x1, *x2) {
                grid[x as usize][*y1 as usize] += 1;
                //grid[(x * y_size + *y1) as usize] += 1;
            }
        }
    }
    println!("\tPart 1: {}", grid.iter().flatten().filter(|x| **x > 1).count());
    // ---------- part 2 ----------

    let mut grid = [[0_u32; 1000]; 1000];
    for ((x1, y1), (x2, y2)) in &lines {
        if x1 == x2 {
            for y in min(*y1, *y2)..=max(*y1, *y2) {
                //grid[(*x1 * y_size + y) as usize] += 1;
                grid[*x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            for x in min(*x1, *x2)..=max(*x1, *x2) {
                grid[x as usize][*y1 as usize] += 1;
                //grid[(x * y_size + *y1) as usize] += 1;
            }
        } else {
            let (start_y, negative) = if *x1 < *x2 {
                (*y1, y2 < y1)
            } else {
                (*y2, y1 < y2)
            };
            // diagonal line
            for (i, x) in (min(*x1, *x2)..=max(*x1, *x2)).into_iter().enumerate() {
                let y = start_y + if negative { -(i as i64) } else { i as i64 };
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    println!("\tPart 2: {}", grid.iter().flatten().filter(|x| **x > 1).count());
}
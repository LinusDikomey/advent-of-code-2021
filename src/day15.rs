use std::{collections::{HashSet, HashMap}, usize};

pub fn run(input: &str) {
    let mut grid = [[0; 100]; 100];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c as u8 - '0' as u8;
        }
    }
    println!("Day 15:");
    // ---------- part 1 ----------
    
    println!("\tPart 1: {}", find_path((0, 0), (99, 99), &grid));
    // ---------- part 2 ----------
    let mut new_grid = [[0; 500]; 500];
    fn copy_tile(start_x: usize, start_y: usize, grid: &mut [[u8; 500]; 500], tile: &[[u8; 100]; 100], add: u8) {
        for y in 0..100 {
            for x in 0..100 {
                let mut val = tile[y][x] + add;
                if val > 9 { val -= 9; }
                grid[start_y + y][start_x + x] = val;
            }
        }
    }

    for y in 0..5 {
        for x in 0..5 {
            copy_tile(x * 100, y * 100, &mut new_grid, &grid, (x+y) as u8);
        }
    }
    println!("\tPart 2: {}", find_path((0, 0), (499, 499), &new_grid));
}

struct Node {
    g_cost: u32,
    h_cost: u32
}
impl Node {
    fn f_cost(&self) -> u32 { self.g_cost + self.h_cost }
}

fn find_path<const X: usize, const Y: usize>(start: (i32, i32), end: (i32, i32), grid: &[[u8; X]; Y]) -> u32 {
    let mut open = HashMap::new();
    open.insert(start, Node { g_cost: 0, h_cost: 0 });
    let mut closed = HashSet::new();

    while open.len() > 0 {
        let (current_pos, _) = open.iter().min_by(|(_, a), (_, b)| match a.f_cost().cmp(&b.f_cost()) {
            std::cmp::Ordering::Equal => a.h_cost.cmp(&b.h_cost),
            ord => ord
        }).unwrap();
        let current_pos = *current_pos;
        let current = open.remove(&current_pos).unwrap();
        closed.insert(current_pos);

        if current_pos == end {
            return current.g_cost;
        }

        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            .map(|(x, y)| (current_pos.0 + x, current_pos.1 + y))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < X as i32 && *y < Y as i32)
            .filter(|p| !closed.contains(p));
        for (x, y) in neighbors {
            let new_cost = current.g_cost + grid[y as usize][x as usize] as u32;
            if let Some(neighbor) = open.get_mut(&(x, y)) {
                if new_cost < neighbor.g_cost {
                    neighbor.g_cost = new_cost;
                }
            } else {
                open.insert((x, y), Node { g_cost: new_cost, h_cost: distance((x, y), end)});
            }
        }
    }
    panic!("No path found")
}

fn distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    (a.0 - b.0).abs() as u32 + (a.1 - b.1).abs() as u32
}
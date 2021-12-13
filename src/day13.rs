use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis { X, Y }
use Axis::*;

pub fn run(input: &str) {
    let mut lines = input.lines();
    let mut points: HashSet<(u32, u32)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut s = line.split(',');
            (s.next().unwrap().parse().unwrap(), s.next().unwrap().parse().unwrap())
        })
        .collect();
    let folds: Vec<(Axis, u32)> = lines.map(|line|
        if line.starts_with("fold along x=") {
            (X, line.split_at("fold along x=".len()).1.parse().unwrap())
        } else if line.starts_with("fold along y=") {
            (Y, line.split_at("fold along y=".len()).1.parse().unwrap())
        } else {
            panic!()
        }
    )
    .collect::<Vec<_>>();
    println!("Day 13:");
    // ---------- part 1 ----------
    
    println!("\tPart 1: {}", fold(&points, folds[0]).len());
    // ---------- part 2 ----------
    for f in folds {
        points = fold(&points, f);
    }
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();
    println!("\tPart 2:");
    for y in 0..=max_y {
        let line = (0..=max_x).map(|x| if points.contains(&(x, y)) {'#'} else {' '}).collect::<String>();
        println!("{}", line);
    }
}

fn fold(points: &HashSet<(u32, u32)>, fold: (Axis, u32)) -> HashSet<(u32, u32)> {
    points.iter()
        .map(|(x, y)|
            match fold.0 {
                X if *x > fold.1 => (2*fold.1 -*x, *y),
                Y if *y > fold.1 => (*x, 2*fold.1 -*y),
                _ => (*x, *y)
            }
        )
        .collect()
}
#[derive(Clone)]
enum SnailNumber {
    Number(u8),
    Pair(Box<SnailNumber>, Box<SnailNumber>)
}
use std::str::Chars;

use SnailNumber::*;
impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n),
            Pair(l, r) => write!(f, "[{}, {}]", l, r)
        }
        
    }
}
impl SnailNumber {
    fn parse(s: &mut Chars) -> Self {
        match s.next().unwrap() {
            '[' => {
                let l = Box::new(Self::parse(s));
                let comma = s.next().unwrap();
                assert_eq!(comma, ',');
                let r = Box::new(Self::parse(s));
                let p = Pair(l, r);
                let closing = s.next().unwrap();
                assert_eq!(closing, ']');
                p
            },
            c => Number(c.to_string().parse().unwrap())
        }
    }
    fn unwrap_number(&mut self) -> u8 {
        match self {
            Number(n) => *n,
            Pair(_, _) => panic!("Tried to unwrap pair as number!")
        }
    }
    fn add_l(&mut self, x: u8) {
        match self {
            Number(n) => *n += x,
            Pair(l, _) => l.add_l(x)
        }
    }
    fn add_r(&mut self, x: u8) {
        match self {
            Number(n) => *n += x,
            Pair(_, r) => r.add_r(x)
        }
    }
    fn try_explode(&mut self, depth: usize) -> Option<(Option<u8>, Option<u8>)> {
        match self {
            Number(_) => None,
            Pair(l, r) => if depth == 4 {
                let res = Some((Some(l.unwrap_number()), Some(r.unwrap_number())));
                *self = Number(0);
                res
            } else {
                if let Some((exploded_l, mut exploded_r)) = l.try_explode(depth + 1) {
                    if let Some(exploded_r) = exploded_r.take() {
                        r.add_l(exploded_r);
                    }
                    Some((exploded_l, exploded_r))
                } else {
                    if let Some((mut exploded_l, exploded_r)) = r.try_explode(depth + 1) {
                        if let Some(exploded_l) = exploded_l.take() {
                            l.add_r(exploded_l);
                        }
                        Some((exploded_l, exploded_r))
                    } else {
                        None
                    }
                }
            }
        }
    }
    fn try_split(&mut self) -> bool {
        match self {
            Number(n) => if *n >= 10 {
                *self = Pair(Box::new(Number(*n/2)), Box::new(Number(*n/2 + if *n % 2 == 1 {1} else {0})));
                true
            } else {
                false
            },
            Pair(l, r) => l.try_split() || r.try_split()
        }
    }
    fn reduce(&mut self) {
        loop {
            if self.try_explode(0).is_some() { continue; }
            if self.try_split() { continue; }
            break;
        }
    }
    fn magnitude(&self) -> u64 {
        match self {
            Number(n) => *n as u64,
            Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude()
        }
    }
}

fn add(l: SnailNumber, r: SnailNumber) -> SnailNumber {
    let mut new = SnailNumber::Pair(Box::new(l), Box::new(r));
    new.reduce();
    new
}

pub fn run(input: &str) {
    let input = input.lines().map(|line| SnailNumber::parse(&mut line.chars())).collect::<Vec<_>>();
    println!("Day 18:");
    // ---------- part 1 ----------
    println!("\tPart 1: {}", input.iter().cloned().reduce(|l, r| add(l, r)).unwrap().magnitude());
    // ---------- part 2 ----------
    println!("\tPart 2: {}", input.iter().cloned().enumerate()
        .map(|(i, l)| input.iter().cloned().enumerate()
            .filter(move |(i2, _)| i != *i2)
            .map(move |(_, r)| add(l.clone(), r).magnitude())
        )
        .flatten()
        .max().unwrap()
    );
}
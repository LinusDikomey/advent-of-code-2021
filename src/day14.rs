use std::collections::HashMap;

pub fn run(input: &str) {
    let (template, rules_str) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    for line in rules_str.lines() {
        let (k, v) = line.split_once(" -> ").unwrap();
        let mut c = k.chars();
        rules.insert([c.next().unwrap(), c.next().unwrap()], v.chars().next().unwrap());

    }

    println!("Day 14:");
    // ---------- part 1 ----------
    println!("\tPart 1: {}", run_steps(template.to_owned(), &rules, 10));
    // ---------- part 2 ----------
    println!("\tPart 2: {}", run_steps2(template, &rules, 40));
}

fn run_steps2(template: &str, rules: &HashMap<[char; 2], char>, steps: usize) -> u64 {
    let mut counts = [0; 26];
    let mut mem = HashMap::new();
    for c in template.chars() {
        counts[(c as u8 - 'A' as u8) as usize] += 1;
    }
    for i in 0..template.chars().count() - 1 {
        let l = template.chars().nth(i).unwrap();
        let r = template.chars().nth(i+1).unwrap();
        steps_recursive(l, r, rules, steps, &mut counts, &mut mem);
    }
    *counts.iter().max().unwrap() - *counts.iter().filter(|e| **e != 0).min().unwrap()
}

fn steps_recursive(a: char, b: char, rules: &HashMap<[char; 2], char>, steps: usize, counts: &mut [u64; 26], mem: &mut HashMap<(char, char, usize), [u64; 26]>) {
    if steps == 0 { return; }
    if let Some(memorized) = mem.get(&mut (a, b, steps)) {
        for (i, c) in memorized.iter().enumerate() {
            counts[i] += c;
        }
        return;
    }
    let new = rules[&[a, b]];
    let mut current_counts = [0; 26];
    current_counts[(new as u8 - 'A' as u8) as usize] += 1;
    steps_recursive(a, new, rules, steps - 1, &mut current_counts, mem);
    steps_recursive(new, b, rules, steps - 1, &mut current_counts, mem);
    for (i, c) in current_counts.iter().enumerate() {
        counts[i] += c;
    }
    mem.insert((a, b, steps), current_counts);
}

fn run_steps(mut polymer: String, rules: &HashMap<[char; 2], char>, steps: usize) -> u64 {
    for _ in 0..steps {
        polymer = step(&polymer, &rules);
    }
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for c in polymer.chars() {
        *char_counts.entry(c).or_default() += 1;
    }
    char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
}

fn step(polymer: &str, rules: &HashMap<[char; 2], char>) -> String {
    let mut res = String::with_capacity(polymer.len() * 2 - 1);
    res.push(polymer.chars().next().unwrap());
    for i in 0..polymer.chars().count() - 1 {
        let pair = [
            polymer.chars().nth(i).unwrap(),
            polymer.chars().nth(i+1).unwrap()
        ];

        if let Some(new) = rules.get(&pair) {
            res.push(*new);
        } else {
            panic!();
        }
        res.push(pair[1]);
    }
    res
}
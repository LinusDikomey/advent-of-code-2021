use std::str::Chars;

pub fn run(input: &str) {
    fn matching(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None
        }
    }
    fn error_score(c: char) -> u64 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!()
        }
    }

    println!("Day 10:");
    // ---------- part 1 ----------
    
    fn first_error(chars: &mut Chars, closing: Option<char>) -> Result<(), u64> {
        while let Some(c) = chars.next() {
            if let Some(closing) = closing {
                if c == closing {
                    break;
                }
            }
            match matching(c) {
                Some(m) => first_error(chars, Some(m))?,
                None => return Err(error_score(c))
            }
        }
        Ok(())
    }

    println!("\tPart 1: {}", 
        input.lines()
            .map(|line| if let Err(score) = first_error(&mut line.chars(), None) { score } else { 0 })
            .sum::<u64>()
    );
    // ---------- part 2 ----------

    fn missing_points(c: char) -> u64 {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!()
        }
    }
    fn missing_chars(chars: &mut Chars, closing: Option<char>) -> String {
        let mut current_missing = String::new();
        loop {
            match chars.next() {
                None => {
                    if let Some(c) = closing {
                        current_missing.push(c);
                    }
                    break;
                },
                Some(c) if matching(c).is_some() => {
                    current_missing.push_str(&missing_chars(chars, matching(c)));
                },
                Some(c) if c == closing.unwrap() => {
                    break;
                },
                _ => unreachable!("Should have been filtered in part 1")
            }
        }
        current_missing
    }
    let mut incomplete = input.lines()
        .filter(|line| first_error(&mut line.chars(), None).is_ok())
        .map(|line| missing_chars(&mut line.chars(), None).chars().fold(0, |prev, c| prev * 5 + missing_points(c)))
        .collect::<Vec<_>>();
    incomplete.sort_unstable();
    
    println!("\tPart 2: {}", incomplete[incomplete.len() / 2]);
}

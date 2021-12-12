use std::iter::once;

pub fn run(input: &str) {
    let paths: Vec<[&str; 2]> = input.lines().map(|l| {
        let mut s = l.split('-');
        [s.next().unwrap(), s.next().unwrap()]
    }).collect();

    println!("Day 12:");
    // ---------- part 1 ----------
    
    println!("\tPart 1: {}", all_paths("start", &paths, Part1Visit { visited: Vec::new() }).len());
    // ---------- part 2 ----------

    println!("\tPart 2: {}", all_paths("start", &paths, Part2Visit { visited: Vec::new(), extra: None }).len());
}

fn large(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase() || s == "end"
}

trait VisitCondition<'a> : Clone {
    fn visit(&mut self, cave: &'a str);
    fn may_visit(&self, cave: &'a str) -> bool;
}

fn all_paths<'a, V: VisitCondition<'a>>(current: &'a str, paths: &[[&'a str; 2]], mut v: V) -> Vec<Vec<&'a str>> {
    v.visit(current);
    if current == "end" {
        return vec![vec!["end"]];
    }
    let out = paths.iter()
        .filter_map(|[a, b]|
            if *a == current { Some(b) }
            else if *b == current { Some(a) }
            else { None }
        )
        .filter(|next| v.may_visit(next))
        .map(|next|
            all_paths(next, paths, v.clone())
                .into_iter()
                .map(|new_path| once(current).chain(new_path.into_iter()).collect())
        )
        .flatten().collect();
    out
}

#[derive(Clone)]
struct Part1Visit<'a> {
    visited: Vec<&'a str>
}
impl<'a> VisitCondition<'a> for Part1Visit<'a> {
    fn visit(&mut self, cave: &'a str) {
        if !large(cave) {
            self.visited.push(cave)
        }
    }
    fn may_visit(&self, cave: &'a str) -> bool {
        !self.visited.contains(&cave)
    }
}

#[derive(Clone)]
struct Part2Visit<'a> {
    visited: Vec<&'a str>,
    extra: Option<&'a str>
}
impl<'a> VisitCondition<'a> for Part2Visit<'a> {
    fn visit(&mut self, cave: &'a str) {
        if large(cave) { return; }
        if self.visited.contains(&cave) {
            self.extra = Some(cave);
        } else {
            self.visited.push(cave);
        }
    }
    fn may_visit(&self, cave: &'a str) -> bool {
        large(cave) || !self.visited.contains(&cave) || (self.extra.is_none() && cave != "start")
    }
}
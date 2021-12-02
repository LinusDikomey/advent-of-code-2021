fn main() {
    // Part 1
    let mut depth = 0;
    let mut pos = 0;
    for line in include_str!("input.txt").lines() {
        let mut segments = line.split(' ');
        let command = segments.next().unwrap();
        let num = segments.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => pos += num,
            "up" => depth -= num,
            "down" => depth += num,
            _ => panic!()
        }
    }
    println!("Part 1: {}", pos * depth);

    // Part 2

    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for line in include_str!("input.txt").lines() {
        let mut segments = line.split(' ');
        let command = segments.next().unwrap();
        let num = segments.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => {
                pos += num;
                depth += num * aim;
            },
            "up" => aim -= num,
            "down" => aim += num,
            _ => panic!()
        }
    }
    println!("Part 2: {}", pos * depth);

    nicer_solution();
}

fn nicer_solution() {
    enum Command {
        Forward(i32),
        Up(i32),
        Down(i32)
    }
    use Command::*;
    impl From<&str> for Command {
        fn from(s: &str) -> Self {
            let mut segments = s.split(' ');
            let command = segments.next().unwrap();
            let num = segments.next().unwrap().parse::<i32>().unwrap();
            match command {
                "forward" => Forward(num),
                "up" => Up(num),
                "down" => Down(num),
                _ => panic!()
            }
        }
    }
    let commands: Vec<Command> = include_str!("input.txt").lines().map(Command::from).collect();

    // Part 1
    let mut depth = 0;
    let mut pos = 0;
    for command in &commands {
        match command {
            Up(x) => depth -= x,
            Down(x) => depth += x,
            Forward(x) => pos += x
        }
    }
    println!("Part 1: {}", depth * pos);

    // Part 2
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for command in &commands {
        match command {
            Up(x) => aim -= x,
            Down(x) => aim += x,
            Forward(x) => {
                pos += x;
                depth += x * aim;
            }
        }
    }
    println!("Part 2: {}", depth * pos);
}
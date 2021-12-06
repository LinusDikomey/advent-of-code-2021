pub fn run(input: &str) {

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
    let commands: Vec<Command> = input.lines().map(Command::from).collect();

    println!("Day 2:");
    // ---------- part 1 ----------
    
    let mut depth = 0;
    let mut pos = 0;
    for command in &commands {
        match command {
            Up(x) => depth -= x,
            Down(x) => depth += x,
            Forward(x) => pos += x
        }
    }

    println!("\tPart 1: {}", depth * pos);
    // ---------- part 2 ----------

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

    println!("\tPart 2: {}", depth * pos);
}
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
    println!("Part 1: {}", pos * depth);
}

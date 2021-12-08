pub fn run(input: &str) {
    let rows: Vec<([u8; 10], [u8; 4])> = input.lines()
        .map(|line| {
            let mut in_out = line.split('|');
            let mut inp = in_out.next().unwrap().trim().split(' ');
            let mut out = in_out.next().unwrap().trim().split(' ');
            let mut in_segments = [0; 10];
            let mut out_segments = [0; 4];
            for i in 0..10 {
                in_segments[i] = parse_seven_seg(inp.next().unwrap());
            }
            for i in 0..4 {
                out_segments[i] = parse_seven_seg(out.next().unwrap());
            }
            (in_segments, out_segments)
        }).collect();
    println!("Day 8:");
    // ---------- part 1 ----------
    println!("\tPart 1: {}", rows.iter().map(|(_, out)| 
        out.iter().map(|x| *x).filter_map(simple_digit).count()).sum::<usize>()
    );
    // ---------- part 2 ----------
    println!("\tPart 2: {}", rows.iter().map(line_number).sum::<usize>());
}

fn parse_seven_seg(s: &str) -> u8 {
    s.chars().map(|c| 1 << (c as u8 - b'a')).reduce(|a, b| a|b).unwrap()
}

fn simple_digit(x: u8) -> Option<u8> {
    match x.count_ones() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None
    }
}

fn line_number((inp, out): &([u8; 10], [u8; 4])) -> usize {
    let mut s = [0_u8; 10];
    let mut length_5 = Vec::new();
    let mut length_6 = Vec::new();
    
    for segments in inp {
        if let Some(simple) = simple_digit(*segments) {
            s[simple as usize] = *segments;
        }
    }
    for segments in inp {
        let segments = *segments;
        if simple_digit(segments).is_some() {
            continue;
        }
        match segments.count_ones() {
            5 => length_5.push(segments),
            6 => length_6.push(segments),
            _ => unreachable!()
        }
    }
    // we use the trivial numbers to isolate segments
    let s1 = s[1];
    let s7 = s[7];
    let bcdf = s[4];
    let eg = s[8] - bcdf - (s[7] - s[1]);
    s[2] = length_5.drain_filter(|elem| *elem & eg == eg).next().unwrap();
    s[3] = length_5.drain_filter(|elem| *elem & s7 == s7).next().unwrap();
    s[5] = length_5[0];
    
    s[9] = length_6.drain_filter(|elem| *elem & bcdf == bcdf).next().unwrap();
    s[0] = length_6.drain_filter(|elem| *elem & s1 == s1).next().unwrap();
    s[6] = length_6[0];
    out.iter()
        .rev()
        .enumerate()
        .map(|(num_index, segments)| 
            s.iter().enumerate().find(|(_, search)| *segments == **search).unwrap().0 * 10_usize.pow(num_index as u32)
        )
        .sum()
}
#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    content: Content
}
impl Packet {
    fn version_sum(&self) -> u64 {
        self.version as u64 + match &self.content {
            Content::Literal(_) => 0,
            Content::Operator(children) => children.iter().map(|child| child.version_sum()).sum()
        }
    }
    fn result(&self) -> i64 {
        match &self.content {
            Content::Literal(lit) => lit.iter().flatten().rev().enumerate().map(|(i, b)| if *b {1 << i} else {0}).sum(),
            Content::Operator(packets) => {
                let mut it = packets.iter().map(|p| p.result());
                match self.type_id {
                    0 => it.sum(),
                    1 => it.product(),
                    2 => it.min().unwrap(),
                    3 => it.max().unwrap(),
                    id@5..=7 => {
                        let f = match id {
                            5 => i64::gt,
                            6 => i64::lt,
                            7 => i64::eq,
                            _ => unreachable!()
                        };
                        f(&it.next().unwrap(), &it.next().unwrap()) as i64                        
                    }
                    _ => unreachable!()
                }
            }

        }
    }
}

#[derive(Debug, PartialEq)]
enum Content {
    Literal(Vec<[bool; 4]>),
    Operator(Vec<Box<Packet>>)
}

type I<'a> = std::iter::Peekable<std::slice::Iter<'a, bool>>;

pub fn run(input: &str) {
    let binary_input = hex_to_bits(input);
    let packet = parse_packet(&mut binary_input.iter().peekable());

    println!("Day 16:");
    // ---------- part 1 ----------
    println!("\tPart 1: {}", packet.version_sum());
    // ---------- part 2 ----------
    println!("\tPart 2: {}", packet.result());
}

fn hex_to_bits(hex: &str) -> Vec<bool> {
    hex.trim().chars().flat_map(|c| {
        let n = u8::from_str_radix(&c.to_string(), 16).unwrap();
        [
            n & 1 << 3 > 0,
            n & 1 << 2 > 0,
            n & 1 << 1 > 0,
            n & 1 << 0 > 0
        ]
    }).collect::<Vec<bool>>()
}

fn parse_packet(p: &mut I) -> Packet {
    let version = parse_num(p, 3) as u8;
    let type_id = parse_num(p, 3) as u8;
    let content = match type_id {
        4 => Content::Literal(parse_lit(p)),
        _ => {
            match p.next().unwrap() {
                false => {
                    let bit_count = parse_num(p, 15);
                    let bits = p.take(bit_count as usize).map(|b| *b).collect::<Vec<_>>();
                    let mut inner_p = bits.iter().peekable();
                    let mut packets = Vec::new();
                    while inner_p.peek().is_some() {
                        packets.push(Box::new(parse_packet(&mut inner_p)));
                    }
                    Content::Operator(packets)
                },
                true => Content::Operator((0..parse_num(p, 11)).map(|_| Box::new(parse_packet(p))).collect()),
            }
        }
    };

    Packet { version, type_id, content }
}

fn parse_lit(p: &mut I) -> Vec<[bool; 4]> {
    let mut keep_reading = true;
    let mut bits = Vec::new();
    while keep_reading {
        keep_reading = *p.next().unwrap();
        let mut bit_group = [false; 4];
        for (i, b) in p.take(4).enumerate() {
            bit_group[i] = *b;
        }
        bits.push(bit_group);
    }
    bits
}

fn parse_num(p: &mut I, n: usize) -> u32 {
    p.take(n).collect::<Vec<_>>().iter().rev().enumerate().map(|(i, b)| if **b { 1 << i } else { 0 }).sum()
}
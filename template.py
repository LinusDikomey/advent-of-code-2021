#!/usr/bin/python3
import os

mods = ""
days = ""
day_pats = ""

if input('This will override files in the src directory. Type "generate" to proceed at your own risk: ') != 'generate':
    print('Cancelling execution')
    exit()

if not os.path.exists("src/input"):
    os.makedirs("src/input")

for i in range(1, 26):
    mods += f"mod day{i};\n"
    days += f'\t\tday{i}::run(include_str!("input/day{i}.txt"));\n'
    day_pats += f'\t\t\t"{i}" => day{i}::run(include_str!("input/day{i}.txt")),\n'
    f = open(f"src/day{i}.rs", 'w+')
    f.write(f"""pub fn run(_input: &str) {{
    println!("Day {i}:");
    // ---------- part 1 ----------
    
    println!("\\tPart 1: {{}}", 0);
    // ---------- part 2 ----------

    println!("\\tPart 2: {{}}", 0);
}}""")
    input = open(f"src/input/day{i}.txt", 'w+')

main = open("src/main.rs", 'w+')
main.write(f"""
{mods}

fn main() {{
    let arg = std::env::args().skip(1).next();
    if let Some(arg) = arg {{
        match arg.as_str() {{
{day_pats}\t\t_ => panic!("Invalid day input!")
        }}
    }} else {{
{days}\t}}
}}
""")
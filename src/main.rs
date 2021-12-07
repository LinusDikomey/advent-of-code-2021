
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


fn main() {
    let mut args = std::env::args().skip(1);
	let mut has_args = false;
	while let Some(arg) = args.next() {
		has_args = true;
		match arg.as_str() {
			"1" => day1::run(include_str!("input/day1.txt")),
			"2" => day2::run(include_str!("input/day2.txt")),
			"3" => day3::run(include_str!("input/day3.txt")),
			"4" => day4::run(include_str!("input/day4.txt")),
			"5" => day5::run(include_str!("input/day5.txt")),
			"6" => day6::run(include_str!("input/day6.txt")),
			"7" => day7::run(include_str!("input/day7.txt")),
			"8" => day8::run(include_str!("input/day8.txt")),
			"9" => day9::run(include_str!("input/day9.txt")),
			"10" => day10::run(include_str!("input/day10.txt")),
			"11" => day11::run(include_str!("input/day11.txt")),
			"12" => day12::run(include_str!("input/day12.txt")),
			"13" => day13::run(include_str!("input/day13.txt")),
			"14" => day14::run(include_str!("input/day14.txt")),
			"15" => day15::run(include_str!("input/day15.txt")),
			"16" => day16::run(include_str!("input/day16.txt")),
			"17" => day17::run(include_str!("input/day17.txt")),
			"18" => day18::run(include_str!("input/day18.txt")),
			"19" => day19::run(include_str!("input/day19.txt")),
			"20" => day20::run(include_str!("input/day20.txt")),
			"21" => day21::run(include_str!("input/day21.txt")),
			"22" => day22::run(include_str!("input/day22.txt")),
			"23" => day23::run(include_str!("input/day23.txt")),
			"24" => day24::run(include_str!("input/day24.txt")),
			"25" => day25::run(include_str!("input/day25.txt")),
		_ => panic!("Invalid day input!")
        }
	}
    if !has_args {
		day1::run(include_str!("input/day1.txt"));
		day2::run(include_str!("input/day2.txt"));
		day3::run(include_str!("input/day3.txt"));
		day4::run(include_str!("input/day4.txt"));
		day5::run(include_str!("input/day5.txt"));
		day6::run(include_str!("input/day6.txt"));
		day7::run(include_str!("input/day7.txt"));
		day8::run(include_str!("input/day8.txt"));
		day9::run(include_str!("input/day9.txt"));
		day10::run(include_str!("input/day10.txt"));
		day11::run(include_str!("input/day11.txt"));
		day12::run(include_str!("input/day12.txt"));
		day13::run(include_str!("input/day13.txt"));
		day14::run(include_str!("input/day14.txt"));
		day15::run(include_str!("input/day15.txt"));
		day16::run(include_str!("input/day16.txt"));
		day17::run(include_str!("input/day17.txt"));
		day18::run(include_str!("input/day18.txt"));
		day19::run(include_str!("input/day19.txt"));
		day20::run(include_str!("input/day20.txt"));
		day21::run(include_str!("input/day21.txt"));
		day22::run(include_str!("input/day22.txt"));
		day23::run(include_str!("input/day23.txt"));
		day24::run(include_str!("input/day24.txt"));
		day25::run(include_str!("input/day25.txt"));
	}
}

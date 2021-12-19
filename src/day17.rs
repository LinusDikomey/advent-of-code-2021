use std::ops::RangeInclusive;

pub fn run(_input: &str) {
    let target = (169..=206, -108..=-68);
    println!("Day 17:");
    let (highest, hit_count) = find_highest(target);
    // ---------- part 1 ----------
    println!("\tPart 1: {}", highest);
    // ---------- part 2 ----------
    println!("\tPart 2: {}", hit_count);
}

fn find_highest(target: (RangeInclusive<i32>, RangeInclusive<i32>)) -> (i32, i32) {
    let mut highest = 0;
    let mut hit_count = 0;
    'y_loop: for y_vel in *target.1.start()..1000 {
        //println!("Trying y {}, highest: {}", y_vel, highest);
        'x_loop: for x_vel in 0.. {
            //println!("Trying x {}", x_vel);
            let mut pos = (0, 0);
            let mut vel = (x_vel, y_vel);
            let mut max_y = 0;
            for step in 0.. {
                max_y = std::cmp::max(max_y, pos.1);
                //println!("\tStep {}: pos: {:?}, vel: {:?}", step, pos, vel);
                if target.0.contains(&pos.0) && target.1.contains(&pos.1) {
                    highest = max_y;
                    hit_count += 1;
                    continue 'x_loop;
                }

                if vel.0 == 0 && !target.0.contains(&pos.0) || pos.1 < *target.1.start() {
                    continue 'x_loop;
                }
                simulate_step(&mut pos, &mut vel);
                if step == 0 && pos.0 > *target.0.end() { continue 'y_loop; }
            }
        }
    }
    (highest, hit_count)
}

fn simulate_step(pos: &mut (i32, i32), vel: &mut (i32, i32)) {
    pos.0 += vel.0;
    pos.1 += vel.1;

    vel.0 -= vel.0.signum();
    vel.1 -= 1;
}
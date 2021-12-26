use std::{collections::HashSet, io::Write};

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut ranges = input["target area: x=".len()..input.len()].split(", y=");
    ranges.next();
    let mut range_y = ranges.next().unwrap().split("..");
    let y_min: i32 = range_y.next().unwrap().parse().unwrap();

    let initial_velocity = y_min.abs() - 1;
    let top_position = (initial_velocity * (initial_velocity + 1)) / 2;

    write!(output, "{}", top_position).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let mut ranges = input["target area: x=".len()..input.len()].split(", y=");
    let mut range_x = ranges.next().unwrap().split("..");
    let mut range_y = ranges.next().unwrap().split("..");

    let x_min: i32 = range_x.next().unwrap().parse().unwrap();
    let x_max: i32 = range_x.next().unwrap().parse().unwrap();

    let y_min: i32 = range_y.next().unwrap().parse().unwrap();
    let y_max: i32 = range_y.next().unwrap().parse().unwrap();

    let mut initial_velocities: HashSet<(i32, i32)> = HashSet::with_capacity(1024);

    let mut valid_initial_x_velocities: Vec<(i32, i32)> = Vec::new();

    for x_target in x_min..x_max + 1 {
        let x_min_initial_velocity = ((x_target * 2) as f32).sqrt().floor() as i32;

        let straight_fall_valid = x_min_initial_velocity * (x_min_initial_velocity + 1) / 2 == x_target;
        let straight_fall_ticks = x_min_initial_velocity;

        // we'll handle cases from above the min accumulation state with potentiall infinite ticks to just 2 ticks.
        // Single tick scenario is easier to cover non-generically.
        for x_velocity in x_min_initial_velocity..x_target / 2 + 2 {
            let mut accumulated = 0;
            for (tick, velocity) in (0..x_velocity).map(|v| x_velocity - v).enumerate() {
                accumulated += velocity;
                if accumulated == x_target {
                    valid_initial_x_velocities.push((x_velocity, tick as i32 + 1));
                    break;
                } else if accumulated > x_target {
                    break;
                }
            }
        }

        for y_target in y_min..y_max+1 {
            // The "straight line" is always valid.
            initial_velocities.insert((x_target, y_target));

            for (x_velocity, ticks) in valid_initial_x_velocities.iter() {
                let accumulated_gravity_diff = (ticks * (ticks + 1)) / 2;
                let linear_distance = y_target + accumulated_gravity_diff;
                let remainder = linear_distance % ticks;
                let y_velocity = linear_distance / ticks;
                if remainder == 0 {
                    initial_velocities.insert((*x_velocity, y_velocity));
                }
            }

            if straight_fall_valid {
                let x_velocity = x_min_initial_velocity;
                for ticks in straight_fall_ticks..y_target.abs() * 2 + 1 {
                    let accumulated_gravity_diff = (ticks * (ticks + 1)) / 2;
                    let linear_distance = y_target + accumulated_gravity_diff;
                    let remainder = linear_distance % ticks;
                    let y_velocity = linear_distance / ticks;
                    if remainder == 0 {
                        initial_velocities.insert((x_velocity, y_velocity));
                    }
                }
            }
        }
        valid_initial_x_velocities.clear();
    }

    write!(output, "{}", initial_velocities.len()).unwrap();
}


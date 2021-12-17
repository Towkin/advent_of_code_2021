use std::{fmt::Write, cmp::max};


fn position_at_time(start_velocity: i32, constant_force: i32, time: i32) -> i32 {
    start_velocity * time + ((constant_force * time * time) as f32 * 0.5f32).round() as i32
}

pub fn solve_a(input: &String, output: &mut String) {
    let mut ranges = input["target area: x=".len()..input.len()].split(", y=");
    let mut range_x = ranges.next().unwrap().split("..");
    let mut range_y = ranges.next().unwrap().split("..");

    let x_min: i32 = range_x.next().unwrap().parse().unwrap();
    let x_max: i32 = range_x.next().unwrap().parse().unwrap();

    let y_min: i32 = range_y.next().unwrap().parse().unwrap();
    let y_max: i32 = range_y.next().unwrap().parse().unwrap();

    let initial_velocity = -y_min - 1;
    let top_time_step = initial_velocity;
    let final_time_step = initial_velocity * 2 + 1;

    let top_position = position_at_time(initial_velocity, -1, top_time_step);
    let final_position = position_at_time(initial_velocity, -1, final_time_step);

    let mut position = 0;
    let mut velocity = initial_velocity;
    for _ in 0..top_time_step {
        position += velocity;
        velocity -= 1;

        println!("{}", position);
    }

    println!("Initial velocity: {}, top position: {} (at step {}), final position: {} (at step {})",
        initial_velocity, top_position, top_time_step, final_position, final_time_step);

    write!(output, "{}", top_position).unwrap();
}

pub fn solve_b(input: &String, output: &mut String) {
    write!(output, "{}", 0).unwrap();
}

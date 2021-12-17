use std::fmt::Write;

pub fn solve_a(input: &String, output: &mut String) {
    let mut ranges = input["target area: x=".len()..input.len()].split(", y=");
    ranges.next();
    let mut range_y = ranges.next().unwrap().split("..");
    let y_min: i32 = range_y.next().unwrap().parse().unwrap();

    let initial_velocity = y_min.abs() - 1;
    let top_position = initial_velocity * (initial_velocity + 1) / 2;

    write!(output, "{}", top_position).unwrap();
}

pub fn solve_b(input: &String, output: &mut String) {
    let mut ranges = input["target area: x=".len()..input.len()].split(", y=");
    let mut range_x = ranges.next().unwrap().split("..");
    let mut range_y = ranges.next().unwrap().split("..");

    let x_min: i32 = range_x.next().unwrap().parse().unwrap();
    let x_max: i32 = range_x.next().unwrap().parse().unwrap();

    let y_min: i32 = range_y.next().unwrap().parse().unwrap();
    let y_max: i32 = range_y.next().unwrap().parse().unwrap();

    let x_min_initial_velocity = ((x_min * 2) as f32).sqrt().floor() as i32;
    let x_max_initial_velocity = ((x_max * 2) as f32).sqrt().floor() as i32;

    let y_min_initial_velocity = ((y_max.abs() * 2) as f32).sqrt().floor() as i32 * y_max.signum();
    let y_max_initial_velocity = y_min.abs() - 1;

    let result = (x_max_initial_velocity - x_min_initial_velocity) * (y_max_initial_velocity - y_min_initial_velocity);

    write!(output, "{}", result).unwrap();
}


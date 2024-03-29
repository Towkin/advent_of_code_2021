use std::{cmp::max, io::Write};
use nalgebra::Vector2;

fn get_points(line: &str) -> (Vector2<i32>, Vector2<i32>) {
    let mut values = line.split(" -> ")
        .map(|points| Vector2::from_iterator(points
            .split(',')
            .map(|value|value.parse().unwrap()))
        );
    (values.next().unwrap(), values.next().unwrap())
}

fn points_in_line(origin: Vector2<i32>, vector: Vector2<i32>) -> impl Iterator<Item = Vector2<i32>> {
    let sign_vector: Vector2<i32> = [
        vector[0].signum(),
        vector[1].signum()
    ].into();

    let steps = max(vector[0].abs(), vector[1].abs());
    (0..steps + 1).map(move |step| origin + sign_vector * step)
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let mut map: Box<[u8; 1000*1000]> = Box::new([0; 1000*1000]);

    let mut count = 0;
    for line in lines {
        let (a, b) = get_points(&line);
        let diff = b - a;

        // Only horizontal and vertical
        if diff[0] != 0 && diff[1] != 0 {
            continue;
        }

        for point in points_in_line(a, diff) {
            let index = (point[1] * 1000 + point[0]) as usize;
            map[index] += 1;
            if map[index] == 2 {
                count += 1;
            }
        }
    }

    write!(output, "{}", count).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let mut map: Box<[u8; 1000*1000]> = Box::new([0; 1000*1000]);

    let mut count = 0;
    for line in lines {
        let (a, b) = get_points(&line);
        let diff = b - a;

        for point in points_in_line(a, diff) {
            let index = (point[1] * 1000 + point[0]) as usize;
            map[index] += 1;
            if map[index] == 2 {
                count += 1;
            }
        }
    }

    write!(output, "{}", count).unwrap();
}
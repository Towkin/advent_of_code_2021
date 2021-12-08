use std::{collections::HashMap, cmp::max};

use nalgebra::Vector2;

fn get_points(line: &String) -> (Vector2<i32>, Vector2<i32>) {
    let mut values = line.split(" -> ")
        .map(|points| Vector2::from_iterator(points
            .split(',')
            .map(|value|value.parse().unwrap()))
        );
    (values.next().unwrap(), values.next().unwrap())
}

fn points_in_line(origin: Vector2<i32>, vector: Vector2<i32>) -> impl Iterator<Item = Vector2<i32>> {
    let sign_vector = Vector2::new(
        vector[0].signum(),
        vector[1].signum()
    );

    let steps = max(vector[0].abs(), vector[1].abs());
    (0..steps + 1).map(move |step| origin + sign_vector * step)
}

pub fn solve_day_5a(lines: impl Iterator<Item = String>) -> u32 {
    let mut map: Box<[u32; 1000*1000]> = Box::new([0; 1000*1000]);

    for line in lines {
        let (a, b) = get_points(&line);
        let diff = b - a;

        // Only horizontal and vertical
        if diff[0] != 0 && diff[1] != 0 {
            continue;
        }

        for point in points_in_line(a, diff) {
            map[(point[1] * 1000 + point[0]) as usize] += 1;
        }
    }

    map.iter().filter(|v| **v >= 2).count().try_into().unwrap()
}

pub fn solve_day_5b(lines: impl Iterator<Item = String>) -> u32 {
    let mut map: Box<[u32; 1000*1000]> = Box::new([0; 1000*1000]);

    for line in lines {
        let (a, b) = get_points(&line);
        let diff = b - a;

        for point in points_in_line(a, diff) {
            map[(point[1] * 1000 + point[0]) as usize] += 1;
        }
    }

    map.iter().filter(|v| **v >= 2).count().try_into().unwrap()
}
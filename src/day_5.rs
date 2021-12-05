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

pub fn solve_day_5a(lines: impl Iterator<Item = String>) -> u32 {
    let mut map: HashMap<Vector2<i32>, u32> = HashMap::new();

    for line in lines {
        let (a, b) = get_points(&line);
        let diff = b - a;

        // Only horizontal and vertical
        if diff[0] != 0 && diff[1] != 0 {
            continue;
        }

        let sign_vector = Vector2::new(
            diff[0].signum(),
            diff[1].signum()
        );

        let steps = max(diff[0].abs(), diff[1].abs());
        for step in 0..steps + 1 {
            let v = a + sign_vector * step;
            let count = map.entry(v).or_insert(0);
            *count += 1;
        }
    }

    map.values().filter(|v| **v >= 2).count().try_into().unwrap()
}

pub fn solve_day_5b(lines: impl Iterator<Item = String>) -> u32 {
    let mut map: HashMap<Vector2<i32>, u32> = HashMap::new();

    for line in lines {
        let (a, b) = get_points(&line);
        let diff = b - a;
        let sign_vector = Vector2::new(
            diff[0].signum(),
            diff[1].signum()
        );

        let steps = max(diff[0].abs(), diff[1].abs());
        for step in 0..steps + 1 {
            let v = a + sign_vector * step;
            let count = map.entry(v).or_insert(0);
            *count += 1;
        }
    }

    map.values().filter(|v| **v >= 2).count().try_into().unwrap()
}
use std::{collections::HashSet, io::Write};
use nalgebra::Vector2;

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut folds = input.split("fold along");
    let points = folds.next().unwrap();
    let points = points.lines().filter_map(|point| {
        if point.len() == 0 {
            return None;
        }

        Some(Vector2::from_iterator(
            point.split(',')
                .map(|v| v.parse::<i32>().unwrap())
        ))
    });

    let mut fold = folds.next().unwrap().trim().split('=');
    let axis = fold.next().unwrap();
    let height = fold.next().unwrap().parse().unwrap();
    let points: HashSet<Vector2<i32>> = match axis {
        "x" => HashSet::from_iter(points.map(|point| if point[0] <= height { point } else {
            [height - (point[0] - height), point[1]].into()
        })),
        "y" => HashSet::from_iter(points.map(|point| if point[1] <= height { point } else {
            [point[0], height - (point[1] - height)].into()
        })),
        _ => panic!(),
    };

    write!(output, "{}", points.len()).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let mut folds = input.split("fold along");
    let points = folds.next().unwrap();
    let points = points.lines().filter_map(|point| {
        if point.len() == 0 {
            return None;
        }

        Some(Vector2::from_iterator(
            point.split(',')
                .map(|v| v.parse::<i32>().unwrap())
        ))
    });

    let mut points: Vec<Vector2<i32>> = points.collect();

    for fold in folds {
        let mut fold = fold.trim().split('=');
        let axis = fold.next().unwrap();
        let height: i32 = fold.next().unwrap().parse().unwrap();

        match axis {
            "x" => {
                for point in points.iter_mut() {
                    if point[0] > height {
                        point[0] = height - (point[0] - height);
                    }
                }
            },
            "y" => {
                for point in points.iter_mut() {
                    if point[1] > height {
                        point[1] = height - (point[1] - height);
                    }
                }
            },
            _ => panic!(),
        };
    }

    let max_x = points.iter().map(|p| p[0]).max().unwrap();
    let max_y = points.iter().map(|p| p[1]).max().unwrap();

    let mut point_set: HashSet<Vector2<i32>> = HashSet::new();
    for point in points {
        point_set.insert(point);
    }

    for y in 0..max_y + 1 {
        writeln!(output).unwrap();
        for x in 0..max_x + 1 {
            if point_set.contains(&[x, y].into()) {
                write!(output, "▮").unwrap();
            } else {
                write!(output, " ").unwrap();
            }
        }
    };
}

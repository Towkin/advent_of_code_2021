use std::collections::{HashSet, VecDeque};

pub fn get_height_map<'a>(lines: impl Iterator<Item = &'a str>) -> [u8; 100 * 100] {
    let mut height_map: [u8; 100 * 100] = [0; 100 * 100];
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            height_map[y * 100 + x] = char as u8 - '0' as u8;
        }
    }
    return height_map;
}

pub fn solve_day_9a(input: &String) -> u32 {
    let height_map = get_height_map(input.lines());

    let mut lowest_points: Vec<u8> = Vec::new();
    for y in 0..100 {
        for x in 0..100 {
            let v = height_map[y * 100 + x];
            if y > 0 && v >= height_map[(y - 1) * 100 + x] {
                continue;
            }
            if x > 0 && v >= height_map[y * 100 + x - 1] {
                continue;
            }
            if y < 100 - 1 && v >= height_map[(y + 1) * 100 + x] {
                continue;
            }
            if x < 100 - 1 && v >= height_map[y * 100 + x + 1] {
                continue;
            }
            lowest_points.push(v);
        }
    }

    lowest_points.iter().map(|v| (v + 1) as u32).sum()
}

pub fn get_basin_map<'a>(lines: impl Iterator<Item = &'a str>) -> [bool; 100 * 100] {
    let mut basin_map: [bool; 100 * 100] = [false; 100 * 100];
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            basin_map[y * 100 + x] = (char as u8 - '0' as u8) == 9;
        }
    }
    return basin_map;
}

pub fn solve_day_9b(input: &String) -> u32 {
    let basin_map = get_basin_map(input.lines());

    let mut closed_set: HashSet<(i32, i32)> = HashSet::new();
    let mut open_set: VecDeque<(i32, i32)> = VecDeque::new();

    let mut top_three: [u32; 3] = [0; 3];

    for y in 0..100 {
        for x in 0..100 {
            if basin_map[(y * 100 + x) as usize] {
                continue;
            }
            if closed_set.contains(&(x, y)) {
                continue;
            }

            open_set.push_back((x, y));
            let mut size: u32 = 0;
            while let Some((x, y)) = open_set.pop_front() {
                size += 1;
                let neighbors = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
                for (x, y) in neighbors {
                    if x < 0 || x >= 100 || y < 0 || y >= 100 {
                        continue;
                    }

                    if basin_map[(y * 100 + x) as usize] {
                        continue;
                    }

                    if closed_set.contains(&(x, y))
                    || open_set.contains(&(x, y)) {
                        continue;
                    }

                    open_set.push_back((x, y));
                }

                closed_set.insert((x, y));
            }

            for i in 0..3 {
                if size > top_three[i] {
                    let v = top_three[i];
                    top_three[i] = size;
                    size = v;
                }
            }
        }
    }

    top_three.iter().fold(1, |a, v| a * (*v))
}

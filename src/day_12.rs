use std::collections::HashMap;
use std::fmt::Write;

const MAX_NEIGHBORS: usize = 10;

struct Cave {
    large: bool,
    neighbor_count: u8,
    neighbors: [u8; MAX_NEIGHBORS],
}

fn step(
    cave: &Cave,
    map: &[Cave],
    visited_small_caves: &mut [bool],
    may_visit_cave_twice: bool
) -> u32 {
    let mut ended_paths = 0;
    for i in 0..cave.neighbor_count {
        let cave_index = cave.neighbors[i as usize] as usize;
        let cave = &map[cave_index];
        if cave.neighbor_count == 0 {
            ended_paths += 1;
        } else if cave.large {
            ended_paths += step(&cave, map, visited_small_caves, may_visit_cave_twice);
        } else {
            if !visited_small_caves[cave_index] {
                visited_small_caves[cave_index] = true;
                ended_paths += step(&cave, map, visited_small_caves, may_visit_cave_twice);
                visited_small_caves[cave_index] = false;
            } else if may_visit_cave_twice {
                ended_paths += step(&cave, map, visited_small_caves, false);
            }
        }
    }
    ended_paths
}

fn all_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn make_cave(s: &str) -> Cave {
    Cave {
        large: all_uppercase(s),
        neighbor_count: 0,
        neighbors: [0; MAX_NEIGHBORS],
    }
}

fn solve(input: &String, output: &mut String, may_visit_cave_twice: bool) {
    let mut indices: HashMap<&str, usize> = HashMap::new();
    indices.insert("start", 0);
    indices.insert("end", 1);

    let mut map: Vec<Cave> = vec![
        make_cave("start"),
        make_cave("end"),
    ];

    for line in input.lines() {
        let mut connected_caves = line.split('-');
        let cave_a = connected_caves.next().unwrap();
        let cave_b = connected_caves.next().unwrap();

        let cave_a = *indices.entry(cave_a).or_insert_with(|| {
            let index = map.len();
            map.push(make_cave(cave_a));
            index
        });
        let cave_b = *indices.entry(cave_b).or_insert_with(|| {
            let index = map.len();
            map.push(make_cave(cave_b));
            index
        });

        // If `a` is not the end cave, and `b` is not the start cave
        if cave_a != 1 && cave_b != 0 {
            let cave = &mut map[cave_a];
            cave.neighbors[cave.neighbor_count as usize] = cave_b as u8;
            cave.neighbor_count += 1;
        }

        // If `b` is not the end cave, and `a` is not the start cave
        if cave_b != 1 && cave_a != 0 {
            let cave = &mut map[cave_b];
            cave.neighbors[cave.neighbor_count as usize] = cave_a as u8;
            cave.neighbor_count += 1;
        }
    }

    let mut visited: Vec<bool> = Vec::from_iter((0..map.len()).map(|_| false));

    let paths= step(&map[0], &map, &mut visited, may_visit_cave_twice);

    write!(output, "{}", paths).unwrap();
}

pub fn solve_a(input: &String, output: &mut String) {
    solve(input, output, false);
}

pub fn solve_b(input: &String, output: &mut String) {
    solve(input, output, true);
}
use std::collections::{HashMap, HashSet};

fn all_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn step<'a>(
    next_nodes: &Vec<&'a str>,
    map: &HashMap<&'a str, Vec<&'a str>>,
    visited_small_caves: &mut HashSet<&'a str>,
    may_visit_cave_twice: bool
) -> u32 {
    let mut ended_paths = 0;
    for node in next_nodes {
        if *node == "end" {
            ended_paths += 1;
        } else if all_uppercase(node) {
            ended_paths += step(map.get(node).unwrap(), map, visited_small_caves, may_visit_cave_twice);
        } else {
            if visited_small_caves.insert(node) {
                ended_paths += step(map.get(node).unwrap(), map, visited_small_caves, may_visit_cave_twice);
                visited_small_caves.remove(node);
            } else if may_visit_cave_twice {
                ended_paths += step(map.get(node).unwrap(), map, visited_small_caves, false);
            }
        }
    }
    ended_paths
}

pub fn solve_day_12a(input: &String) -> u32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut connected_caves = line.split('-');
        let cave_a = connected_caves.next().unwrap();
        let cave_b = connected_caves.next().unwrap();

        if cave_a != "end" {
            let cave_a_neighbors = map.entry(cave_a).or_insert_with(Vec::new);
            if cave_b != "start" {
                cave_a_neighbors.push(cave_b);
            }
        }
        if cave_b != "end" {
            let cave_b_neighbors = map.entry(cave_b).or_insert_with(Vec::new);
            if cave_a != "start" {
                cave_b_neighbors.push(cave_a);
            }
        }
    }


    let mut visited: HashSet<&str> = HashSet::new();

    step(
        map.get("start").unwrap(),
        &map, &mut visited, false)
}

pub fn solve_day_12b(input: &String) -> u32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut connected_caves = line.split('-');
        let cave_a = connected_caves.next().unwrap();
        let cave_b = connected_caves.next().unwrap();

        if cave_a != "end" {
            let cave_a_neighbors = map.entry(cave_a).or_insert_with(Vec::new);
            if cave_b != "start" {
                cave_a_neighbors.push(cave_b);
            }
        }
        if cave_b != "end" {
            let cave_b_neighbors = map.entry(cave_b).or_insert_with(Vec::new);
            if cave_a != "start" {
                cave_b_neighbors.push(cave_a);
            }
        }
    }

    let mut visited: HashSet<&str> = HashSet::new();

    step(
        map.get("start").unwrap(),
        &map, &mut visited, true)
}
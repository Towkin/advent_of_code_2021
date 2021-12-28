use std::{io::Write, default};
use nalgebra::Vector3;


#[derive(Default, Clone, Copy)]
struct Region {
    min: Vector3<i32>,
    max: Vector3<i32>,
}

struct RegionUnion {
    len: usize,
    regions: [Region; 4],
}

impl Region {
    fn union(&self, other: Region) -> RegionUnion {
        
    }
}

impl RegionUnion {
    fn new(regions: &[Region]) -> RegionUnion {
        RegionUnion {
            len: regions.len(),
            regions: [
                *regions.get(0).unwrap_or(&Region {..Default::default()}),
                *regions.get(1).unwrap_or(&Region {..Default::default()}),
                *regions.get(2).unwrap_or(&Region {..Default::default()}),
                *regions.get(3).unwrap_or(&Region {..Default::default()}),
            ],
        }
    }
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let steps = input.lines().map(|line| {
        let (state, coordinates) = line.split_once(' ').unwrap();
        let state = match state {
            "on" => true,
            "off" => false,
            _ => panic!(),
        };

        let mut coordinates = coordinates.split(',');
        let coordinates = [
            coordinates.next().unwrap(),
            coordinates.next().unwrap(),
            coordinates.next().unwrap(),
        ]
            .map(|coordinate| coordinate[2..].split_once("..").unwrap())
            .map(|(min, max)| (min.parse::<i32>().unwrap(), max.parse::<i32>().unwrap() + 1));
        
        let min: Vector3<i32> = coordinates.map(|(min, _)| min.clamp(-50, 51) + 50).into();
        let max: Vector3<i32> = coordinates.map(|(_, max)| max.clamp(-50, 51) + 50).into();

        (state, min, max)
    });

    let mut cubes = Box::new([false; 100 * 100 * 100]);
    for (enabled, min, max) in steps {
        for x in min[0]..max[0] {
            for y in min[1]..max[1] {
                for z in min[2]..max[2] {
                    cubes[(x * 100 * 100 + y * 100 + z) as usize] = enabled;
                }
            }
        }
    }
    
    write!(output, "{}", cubes.iter().filter(|v| **v).count()).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let steps: Vec<(bool, Vector3<i32>, Vector3<i32>)> = input.lines().map(|line| {
        let (state, coordinates) = line.split_once(' ').unwrap();
        let state = match state {
            "on" => true,
            "off" => false,
            _ => panic!(),
        };

        let mut coordinates = coordinates.split(',');
        let coordinates = [
            coordinates.next().unwrap(),
            coordinates.next().unwrap(),
            coordinates.next().unwrap(),
        ]
            .map(|coordinate| coordinate[2..].split_once("..").unwrap())
            .map(|(min, max)| (min.parse::<i32>().unwrap(), max.parse::<i32>().unwrap() + 1));
        
        let min: Vector3<i32> = coordinates.map(|(min, _)| min).into();
        let max: Vector3<i32> = coordinates.map(|(_, max)| max).into();

        (state, min, max)
    }).collect();

    let mut enabled_regions: Vec<(Vector3<i32>, Vector3<i32>)> = Vec::new();
    for current_index in 0..steps.len() {
        let (enabled, min, max) = steps[current_index];
        let diff = max - min;
        
        for previous_index in 0..current_index {
            let (was_enabled, previous_min, previous_max) = steps[previous_index];

        }
    }

    write!(output, "{}", 0).unwrap();
}

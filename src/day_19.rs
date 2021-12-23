use std::{collections::{HashMap, HashSet}, fmt::Write};
use nalgebra::{Vector3, Matrix3};

type Position = Vector3<i32>;

struct Transform {
    rotation: Matrix3<i32>,
    translation: Vector3<i32>,
}

impl Transform {
    fn transform(&self, position: Position) -> Position {
        self.rotation * position + self.translation
    }
}

struct Beacon {
    position: Position,
    relative_positions: HashMap<i32, Position>,
}

struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn parse(input: &str) -> Scanner {
        let beacon_positions: Vec<Position> = input.lines().skip(1).map(|line| {
            let mut values = line.split(',');
            [
                values.next().unwrap().parse().unwrap(),
                values.next().unwrap().parse().unwrap(),
                values.next().unwrap().parse().unwrap(),
            ].into()
        }).collect();
        let mut scanner = Scanner {
            beacons: Vec::with_capacity(beacon_positions.len()),
        };
        for beacon_position in beacon_positions.iter() {
            let mut beacon = Beacon {
                position: *beacon_position,
                relative_positions: HashMap::with_capacity(beacon_positions.len()),
            };
            for other_position in beacon_positions.iter() {
                let relative_position = other_position - beacon_position;
                let position_hash = 
                    relative_position[0].abs() + 
                    relative_position[1].abs() + 
                    relative_position[2].abs();

                beacon.relative_positions.insert(position_hash,  relative_position);
            }
            scanner.beacons.push(beacon);
        }

        scanner
    }

    fn get_relative_transform(&self, other: &Scanner) -> Option<Transform> {
        for self_beacon in &self.beacons {
            for other_beacon in &other.beacons {
                'other_positions: for (other_hashed_position, other_relative_position) in other_beacon.relative_positions.iter() {
                    let other_relative_position_abs = other_relative_position.abs();
                    if (other_relative_position_abs[0] == other_relative_position_abs[1]) |
                       (other_relative_position_abs[0] == other_relative_position_abs[2]) |
                       (other_relative_position_abs[1] == other_relative_position_abs[2]) {
                        continue 'other_positions;
                    }

                    if let Some(self_relative_position) = self_beacon.relative_positions.get(&other_hashed_position) {
                        let self_relative_position_abs = self_relative_position.abs();
                        for (x_index, y_index, z_index) in [
                            (0, 1, 2),
                            (0, 2, 1),
                            (1, 0, 2),
                            (1, 2, 0),
                            (2, 0, 1),
                            (2, 1, 0),
                        ] {
                            if (self_relative_position_abs[0] != other_relative_position_abs[x_index]) |
                               (self_relative_position_abs[1] != other_relative_position_abs[y_index]) |
                               (self_relative_position_abs[2] != other_relative_position_abs[z_index]) {
                                continue;
                            }

                            let x_sign = if self_relative_position[0] == other_relative_position[x_index] { 1 } else { -1 };
                            let y_sign = if self_relative_position[1] == other_relative_position[y_index] { 1 } else { -1 };
                            let z_sign = if self_relative_position[2] == other_relative_position[z_index] { 1 } else { -1 };

                            let mut rotation = Matrix3::zeros();
                            rotation[(0, x_index)] = x_sign;
                            rotation[(1, y_index)] = y_sign;
                            rotation[(2, z_index)] = z_sign;
                            let translation = self_beacon.position - rotation * other_beacon.position;
                            let transform = Transform { rotation, translation };

                            let other_positions: Vec<Position> = other.beacons.iter().map(|beacon| transform.transform(beacon.position)).collect();

                            let mut hits = 0;
                            for origin_position in self.beacons.iter().map(|beacon| beacon.position) {
                                if other_positions.contains(&origin_position) {
                                    hits += 1;
                                }
                            }

                            if hits < 12 {
                                continue 'other_positions;
                            }

                            return Some(transform);
                        }
                    }
                }
            }
        }

        None
    }
}

pub fn solve_a(input: &String, output: &mut String) {
    let scanners: Vec<Scanner> = input.split("\n\n").map(Scanner::parse).collect();
    let origin_scanner = &scanners[0];
    let mut scanner_transforms: Vec<Option<Transform>> = Vec::from_iter(scanners.iter().map(|scanner| origin_scanner.get_relative_transform(scanner)));

    while scanner_transforms.iter().any(|t| if let None = t { true} else { false }) {
        for blind_scanner_index in 1..scanner_transforms.len() {
            if let Some(_) = &scanner_transforms[blind_scanner_index] {
                continue;
            }
    
            for known_scanner_index in 1..scanner_transforms.len() {
                if let Some(parent_transform) = &scanner_transforms[known_scanner_index] {
                    if let Some(child_transform) = scanners[known_scanner_index].get_relative_transform(&scanners[blind_scanner_index]) {
                        scanner_transforms[blind_scanner_index] = Some(Transform {
                            rotation: parent_transform.rotation * child_transform.rotation,
                            translation: parent_transform.translation + parent_transform.rotation * child_transform.translation,
                        });
                        break;
                    }
                }
            }
        }
    }

    let mut beacon_positions: HashSet<Position> = HashSet::from_iter(origin_scanner.beacons.iter().map(|beacon| beacon.position));
    for (scanner, transform) in scanners.iter().zip(scanner_transforms) {
        if let Some(transform) = transform {
            for position in scanner.beacons.iter().map(|beacon| transform.transform(beacon.position)) {
                beacon_positions.insert(position);
            }
        }
    }

    write!(output, "{}", beacon_positions.len()).unwrap();
}

pub fn solve_b(input: &String, output: &mut String) {
    let scanners: Vec<Scanner> = input.split("\n\n").map(Scanner::parse).collect();
    let origin_scanner = &scanners[0];
    let mut scanner_transforms: Vec<Option<Transform>> = Vec::from_iter(scanners.iter().map(|scanner| origin_scanner.get_relative_transform(scanner)));

    while scanner_transforms.iter().any(|t| if let None = t { true} else { false }) {
        for blind_scanner_index in 1..scanner_transforms.len() {
            if let Some(_) = &scanner_transforms[blind_scanner_index] {
                continue;
            }
    
            for known_scanner_index in 1..scanner_transforms.len() {
                if let Some(parent_transform) = &scanner_transforms[known_scanner_index] {
                    if let Some(child_transform) = scanners[known_scanner_index].get_relative_transform(&scanners[blind_scanner_index]) {
                        scanner_transforms[blind_scanner_index] = Some(Transform {
                            rotation: parent_transform.rotation * child_transform.rotation,
                            translation: parent_transform.translation + parent_transform.rotation * child_transform.translation,
                        });
                        break;
                    }
                }
            }
        }
    }

    let scanner_positions: Vec<Position> = scanner_transforms.iter().filter_map(|transform| match transform {
        Some(transform) => Some(transform.translation),
        None => None,
    }).collect();
    let longest_manhattan_distance = scanner_positions.iter().map(|a| scanner_positions.iter().map(|b|
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs() ).max().unwrap()).max().unwrap();

    write!(output, "{}", longest_manhattan_distance).unwrap();
}

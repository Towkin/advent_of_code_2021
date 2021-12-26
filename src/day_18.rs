use std::{fmt::{Write as FmtWrite, Display}, ops::Add, io::Write};

#[derive(Clone)]
struct Hierarchy {
    paired: [bool; Hierarchy::SIZE - 1],
    values: [u8; Hierarchy::SIZE],
}

impl Hierarchy {
    const SIZE: usize = 16;
    const PAIR_VALUE_MAP: [usize; Hierarchy::SIZE - 1] = [
        0,
        0, 8,
        0, 4, 8, 12,
        0, 2, 4, 6, 8, 10, 12, 14];
    const VALUE_TOP_PAIR_MAP: [usize; Hierarchy::SIZE] = [
        7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14,
    ];

    fn magnitude(&self) -> u32 {
        fn magnitude_of_pair(hierarchy: &Hierarchy, pair_index: usize) -> u32 {
            let value_index = Hierarchy::PAIR_VALUE_MAP[pair_index];
            if !hierarchy.paired[pair_index] {
                return hierarchy.values[value_index] as u32;
            }

            if pair_index >= (Hierarchy::SIZE - 1) / 2 {
                return hierarchy.values[value_index] as u32 * 3 + hierarchy.values[value_index + 1] as u32 * 2;
            }

            magnitude_of_pair(hierarchy, pair_index * 2 + 1) * 3 +
            magnitude_of_pair(hierarchy, pair_index * 2 + 2) * 2
        }

        magnitude_of_pair(self, 0)
    }

    fn get_previous_value_index(&self, mut index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }

        while index > 0 {
            index -= 1;
            if self.is_valid_value_index(index) {
                return Some(index);
            }
        }

        Some(0)
    }

    fn get_next_value_index(&self, mut index: usize) -> Option<usize> {
        index += 1;
        while index < Hierarchy::SIZE {
            if self.is_valid_value_index(index) {
                return Some(index);
            }

            index += 1;
        }
        None
    }

    fn is_valid_value_index(&self, index: usize) -> bool {
        let mut tree_index = 0;
        let mut offset = 0;
        let mut scope = Hierarchy::SIZE;

        loop {
            if !self.paired[tree_index] {
                return index == offset;
            }

            scope /= 2;
            if scope == 1 {
                return true;
            }

            let right_factor = (index - offset >= scope) as usize;
            offset += scope * right_factor;
            // Left: index * 2 + 1
            // Right: index * 2 + 2
            tree_index = tree_index * 2 + 1 + 1 * right_factor;
        }
    }

    fn parse(line: &str) -> Hierarchy {
        let mut hierarchy = Hierarchy {
            paired: [false; Hierarchy::SIZE - 1],
            values: [0; Hierarchy::SIZE],
        };

        let mut pair_index = 0;
        let mut value_index = 0;
        for c in line.as_bytes().iter() {
            match *c {
                b'[' => {
                    hierarchy.paired[pair_index] = true;
                    pair_index = pair_index * 2 + 1;
                },
                b']' => pair_index = (pair_index - 1) / 2,
                b',' => {
                    pair_index += 1;
                    value_index = hierarchy.get_next_value_index(value_index).unwrap();
                },
                _ => hierarchy.values[value_index] = *c - b'0',
            };
        };

        hierarchy
    }
}

impl Add for Hierarchy {
    type Output = Hierarchy;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        fn explode_top_pairs(left: &mut Hierarchy, right: &mut Hierarchy) -> bool {
            for pair_index in (Hierarchy::SIZE / 2 - 1)..(Hierarchy::SIZE - 1) {
                if left.paired[pair_index] {
                    {
                        let left_value_index = Hierarchy::PAIR_VALUE_MAP[pair_index];
                        if let Some(index) = left.get_previous_value_index(left_value_index) {
                            left.values[index] += left.values[left_value_index];
                        }
                        left.values[left_value_index] = 0;
                    }
                    {
                        let right_value_index = Hierarchy::PAIR_VALUE_MAP[pair_index] + 1;
                        if let Some(index) = left.get_next_value_index(right_value_index) {
                            left.values[index] += left.values[right_value_index];
                        } else {
                            // Move to right
                            right.values[0] += left.values[right_value_index];
                        }
                        left.values[right_value_index] = 0;
                    }

                    left.paired[pair_index] = false;
                    return true;
                }
            }

            for pair_index in (Hierarchy::SIZE / 2 - 1)..(Hierarchy::SIZE - 1) {
                if right.paired[pair_index] {
                    {
                        let left_value_index = Hierarchy::PAIR_VALUE_MAP[pair_index];
                        if let Some(index) = right.get_previous_value_index(left_value_index) {
                            right.values[index] += right.values[left_value_index];
                        } else {
                            let last_index = left.get_previous_value_index(Hierarchy::SIZE).unwrap();
                            // Move to left
                            left.values[last_index] += right.values[left_value_index];
                        }
                        right.values[left_value_index] = 0;
                    }
                    {
                        let right_value_index = Hierarchy::PAIR_VALUE_MAP[pair_index] + 1;
                        if let Some(index) = right.get_next_value_index(right_value_index) {
                            right.values[index] += right.values[right_value_index];
                        }
                        right.values[right_value_index] = 0;
                    }
                    right.paired[pair_index] = false;
                    return true;
                }
            }

            false
        }

        fn split_high_values(hierarchy: &mut Hierarchy) -> bool {
            for value_index in 0..Hierarchy::SIZE {
                if hierarchy.values[value_index] >= 10 {
                    let top_pair_index = Hierarchy::VALUE_TOP_PAIR_MAP[value_index];
                    if hierarchy.paired[top_pair_index] {
                        panic!("No top level pairs should be available at this stage");
                    }

                    let left_value = hierarchy.values[value_index] / 2;
                    let right_value = hierarchy.values[value_index] - left_value;

                    let mut pair_index = top_pair_index;
                    loop {
                        let parent_index = (pair_index - 1) / 2;
                        if hierarchy.paired[parent_index] {
                            break;
                        }
                        pair_index = parent_index;
                    }
                    hierarchy.paired[pair_index] = true;
                    let left_index = value_index;
                    let right_index = hierarchy.get_next_value_index(value_index).unwrap();
                    hierarchy.values[left_index] = left_value;
                    hierarchy.values[right_index] = right_value;

                    return true;
                }
            }

            false
        }

        loop {
            if explode_top_pairs(&mut self, &mut rhs) {
                continue;
            }

            if split_high_values(&mut self) || split_high_values(&mut rhs) {
                continue;
            }

            break;
        }

        let mut output = Hierarchy {
            paired: [false; Hierarchy::SIZE - 1],
            values: [0; Hierarchy::SIZE],
        };

        let scope = Hierarchy::SIZE / 2;
        for value_index in 0..scope {
            output.values[value_index] = self.values[value_index * 2];
            output.values[value_index + scope] = rhs.values[value_index * 2];
        }

        output.paired[0] = true;
        for level in [1, 2, 4] {
            let offset = level - 1;
            let output_left_offset = offset + level;
            let output_right_offset = offset + level * 2;
            for index in 0..level {
                output.paired[output_left_offset + index] = self.paired[offset + index];
                output.paired[output_right_offset + index] = rhs.paired[offset + index];
            }
        }

        output
    }
}

impl Display for Hierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const STEP_SIZE: [usize; 4] = [8, 4, 2, 1];

        fn write_context(
            hierarchy: &Hierarchy,
            f: &mut std::fmt::Formatter<'_>,
            tree_index: usize, value_index: usize, depth: usize) -> std::fmt::Result {
            if tree_index < Hierarchy::SIZE - 1 && hierarchy.paired[tree_index] {
                f.write_char('[')?;
                write_context(hierarchy, f, tree_index * 2 + 1, value_index, depth + 1)?;
                f.write_char(',')?;
                write_context(hierarchy, f, tree_index * 2 + 2, value_index + STEP_SIZE[depth], depth + 1)?;
                f.write_char(']')
            } else {
                Display::fmt(&hierarchy.values[value_index], f)
            }
        }

        write_context(self, f, 0, 0, 0)
    }
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut hierarchy = Hierarchy::parse(input.lines().next().unwrap());
    for line in input.lines().skip(1) {
        hierarchy = hierarchy + Hierarchy::parse(line);
    }

    write!(output, "{}", hierarchy.magnitude()).unwrap();
}


pub fn solve_b(input: &String, output: &mut impl Write) {
    let input: Vec<Hierarchy> = input.lines().map(Hierarchy::parse).collect();

    let mut max: u32 = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                max = std::cmp::max(max, (input[i].clone() + input[j].clone()).magnitude());
            }
        }
    }

    write!(output, "{}", max).unwrap();
}

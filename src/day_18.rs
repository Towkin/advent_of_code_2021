use std::{fmt::{Write, Display, Debug}, ops::Add};

#[derive(Clone, Copy)]
enum Element<Pair = usize, Value = usize> {
    Pair(Pair),
    Value(Value),
}

#[derive(Clone, Copy)]
struct Pair {
    parent: Option<usize>,
    left: Element,
    right: Element,
}

struct Hierarchy {
    values: Vec<u8>,
    pairs: Vec<Pair>,
}

struct Hierarchy2 {
    paired: [bool; Hierarchy2::SIZE - 1],
    values: [u8; Hierarchy2::SIZE],
}

struct Hierarchy2Iterator<'a> {
    index: usize,
    hierarchy: &'a Hierarchy2,
}

impl Hierarchy2 {
    const SIZE: usize = 32;
    const PAIR_VALUE_MAP: [usize; Hierarchy2::SIZE - 1] = [
        0,
        0, 16,
        0, 8, 16, 24,
        0, 4, 8, 12, 16, 20, 24, 28,
        0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30];

    fn iter(&self) -> Hierarchy2Iterator {
        Hierarchy2Iterator {
            index: 0,
            hierarchy: &self,
        }
    }

    fn get_previous_value_index(&self, mut index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }

        while index > 0 {
            if self.is_valid_value_index(index) {
                return Some(index);
            }

            index -= 1;
        }

        Some(0)
    }

    fn get_next_value_index(&self, mut index: usize) -> Option<usize> {
        index += 1;
        while index < Hierarchy2::SIZE {
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
        let mut scope = Hierarchy2::SIZE;

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

    fn parse(line: &str) -> Hierarchy2 {
        let mut hierarchy = Hierarchy2 {
            paired: [false; Hierarchy2::SIZE - 1],
            values: [0; Hierarchy2::SIZE],
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

impl Add for Hierarchy2 {
    type Output = Hierarchy2;

    fn add(self, rhs: Self) -> Self::Output {
        panic!();
    }
}

impl Display for Hierarchy2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const STEP_SIZE: [usize; 5] = [16, 8, 4, 2, 1];

        fn write_context(
            hierarchy: &Hierarchy2,
            f: &mut std::fmt::Formatter<'_>,
            tree_index: usize, value_index: usize, depth: usize) -> std::fmt::Result {
            if tree_index < Hierarchy2::SIZE - 1 && hierarchy.paired[tree_index] {
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

impl<'a> Iterator for Hierarchy2Iterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 16 {
            return None;
        }
        let value = Some(self.hierarchy.values[self.index]);
        if let Some(index) = self.hierarchy.get_next_value_index(self.index) {
            self.index = index;
        } else {
            self.index = 16;
        }
        value
    }
}



fn reduce_explode(pair_index: usize, hierarchy: &mut Hierarchy, depth: u8) -> bool {
    let pair = hierarchy.pairs[pair_index];
    if depth < 4 {
        let element_indices = [pair.left, pair.right];
        for i in 0..2 {
            let element_index = element_indices[i];
            if let Element::Pair(pair_index) = element_index {
                if reduce_explode(pair_index, hierarchy, depth + 1) {
                    return true;
                }
            }
        }
    } else {
        if let Element::Value(left_index) = pair.left {
            if left_index > 0 {
                hierarchy.values[left_index - 1] += hierarchy.values[left_index];
            }
            hierarchy.values[left_index] = 0;

            if let Some(parent_index) = pair.parent {
                if let Element::Pair(index) = hierarchy.pairs[parent_index].left {
                    if index == pair_index {
                        hierarchy.pairs[parent_index].left = Element::Value(left_index);
                    }
                }
                if let Element::Pair(index) = hierarchy.pairs[parent_index].right {
                    if index == pair_index {
                        hierarchy.pairs[parent_index].right = Element::Value(left_index);
                    }
                }
            }
        }

        if let Element::Value(right_index) = pair.right {
            if right_index < hierarchy.values.len() - 1 {
                hierarchy.values[right_index + 1] += hierarchy.values[right_index];
            }
            hierarchy.values.remove(right_index);
            for i in 0..hierarchy.pairs.len() {
                if let Element::Value(index) = hierarchy.pairs[i].left {
                    if index > right_index {
                        hierarchy.pairs[i].left = Element::Value(index - 1);
                    }
                }
                if let Element::Value(index) = hierarchy.pairs[i].right {
                    if index > right_index {
                        hierarchy.pairs[i].right = Element::Value(index - 1);
                    }
                }
            }
        }

        hierarchy.pairs.remove(pair_index);
        for i in 0..hierarchy.pairs.len() {
            if let Element::Pair(index) = hierarchy.pairs[i].left {
                if index > pair_index {
                    hierarchy.pairs[i].left = Element::Pair(index - 1);
                }
            }
            if let Element::Pair(index) = hierarchy.pairs[i].right {
                if index > pair_index {
                    hierarchy.pairs[i].right = Element::Pair(index - 1);
                }
            }
        }

        return true;
    }
    false
}

fn reduce_split(pair_index: usize, hierarchy: &mut Hierarchy) -> bool {
    let element_indices = [hierarchy.pairs[pair_index].left, hierarchy.pairs[pair_index].right];
    for i in 0..2 {
        let element = element_indices[i];
        match element {
            Element::Pair(element_pair_index) => {
                if reduce_split(element_pair_index, hierarchy) {
                    return true;
                }
            }
            Element::Value(value_index) => {
                let value = hierarchy.values[value_index];
                if value >= 10 {
                    let left_value = value / 2;
                    let right_value = value - left_value;

                    let left_value_index = value_index;
                    let right_value_index = value_index + 1;
                    hierarchy.values[left_value_index] = left_value;
                    hierarchy.values.insert(right_value_index, right_value);

                    hierarchy.pairs.push(Pair {
                        parent: Some(pair_index),
                        left: Element::Value(left_value_index),
                        right: Element::Value(right_value_index),
                    });
                    return true;
                }
            }
        }
    }
    false
}

fn reduce(hierarchy: &mut Hierarchy) {
    loop {
        if reduce_explode(0, hierarchy, 0) {
            continue;
        }

        if reduce_split(0, hierarchy) {
            continue;
        }

        break;
    }
}

fn combine(left: Hierarchy, right: Hierarchy) -> Hierarchy {
    let mut hierarchy = Hierarchy {
        pairs: Vec::with_capacity(1 + left.pairs.len() + right.pairs.len()),
        values: left.values,
    };
    let right_value_offest = hierarchy.values.len();
    for value in right.values {
        hierarchy.values.push(value);
    }

    let left_pair_offset = 1;
    let right_pair_offset = left.pairs.len() + 1;
    hierarchy.pairs.push(Pair {
        parent: None,
        left: Element::Pair(left_pair_offset),
        right: Element::Pair(right_pair_offset),
    });

    for (pairs, pair_offset, value_offset) in [
        (left.pairs, left_pair_offset, 0),
        (right.pairs, right_pair_offset, right_value_offest)
    ] {
        for pair in pairs {
            hierarchy.pairs.push(Pair {
                parent: match pair.parent {
                    Some(parent_index) => Some(parent_index + pair_offset),
                    None => Some(0),
                },
                left: match pair.left {
                    Element::Pair(pair_index) => Element::Pair(pair_index + pair_offset),
                    Element::Value(value_index) => Element::Value(value_index + value_offset),
                },
                right: match pair.right {
                    Element::Pair(pair_index) => Element::Pair(pair_index + pair_offset),
                    Element::Value(value_index) => Element::Value(value_index + value_offset),
                },
            });
        }
    }


    hierarchy
}

impl Display for Hierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pairs[0].fmt(self, f)
    }
}

impl Pair {
    fn fmt(&self, hierarchy: &Hierarchy, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        match &self.left {
            Element::Pair(pair) => hierarchy.pairs[*pair].fmt(hierarchy, f),
            Element::Value(value) => panic!(),//hierarchy.values[*value].fmt(f),
        }?;
        f.write_char(',')?;
        match &self.right {
            Element::Pair(pair) => hierarchy.pairs[*pair].fmt(hierarchy, f),
            Element::Value(value) => panic!(),//hierarchy.values[*value].fmt(f),
        }?;
        f.write_char(']')
    }
}

enum Side {
    Left,
    Right,
}

fn parse_line(line: &str) -> Hierarchy {
    let mut hierarchy = Hierarchy {
        pairs: Vec::new(),
        values: Vec::new(),
    };
    let mut current_pair_index = 0;
    let mut side = Side::Left;

    const PAIR_START: u8 = '[' as u8;
    const PAIR_END: u8 = ']' as u8;
    const PAIR_SEPARATOR: u8 = ',' as u8;

    for c in line.as_bytes().iter() {
        match *c {
            PAIR_START => {
                let child_pair_index = hierarchy.pairs.len();
                if current_pair_index >= child_pair_index {
                    // Root
                    hierarchy.pairs.push(Pair {
                        parent: None,
                        left: Element::Value(0),
                        right: Element::Value(0),
                    });
                } else {
                    hierarchy.pairs.push(Pair {
                        parent: Some(current_pair_index),
                        left: Element::Value(0),
                        right: Element::Value(0),
                    });
                    let child_element = Element::Pair(child_pair_index);
                    match side {
                        Side::Left => hierarchy.pairs[current_pair_index].left = child_element,
                        Side::Right => hierarchy.pairs[current_pair_index].right = child_element,
                    }
                }
                side = Side::Left;
                current_pair_index = child_pair_index;
            },
            PAIR_END => {
                if let Some(parent_index) = hierarchy.pairs.last().unwrap().parent {
                    current_pair_index = parent_index;
                } else {
                    break;
                }
            },
            PAIR_SEPARATOR => {
                side = Side::Right;
            },
            _ => {
                let value = *c - '0' as u8;
                let value_index = Element::Value(hierarchy.values.len());
                hierarchy.values.push(value);

                match side {
                    Side::Left => hierarchy.pairs[current_pair_index].left = value_index,
                    Side::Right => hierarchy.pairs[current_pair_index].right = value_index,
                }
            },
        };
    };

    hierarchy
}

pub fn solve_a(input: &String, output: &mut String) {
    let hierarchy = Hierarchy2::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    println!("{}", hierarchy);
    // let mut hierarchy = parse_line("[[[[[9,8],1],2],3],4]");
    // reduce(&mut hierarchy);
    // println!("{}", hierarchy);

    // let hierarchies = input.lines().map(parse_line);

    // for hierarchy in hierarchies {
    //     println!("{}", hierarchy);
    // }

    write!(output, "{}", 0).unwrap();
}


pub fn solve_b(input: &String, output: &mut String) {
    write!(output, "{}", 0).unwrap();
}

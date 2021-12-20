use std::fmt::{Write, Display};

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
            Element::Value(value) => hierarchy.values[*value].fmt(f),
        }?;
        f.write_char(',')?;
        match &self.right {
            Element::Pair(pair) => hierarchy.pairs[*pair].fmt(hierarchy, f),
            Element::Value(value) => hierarchy.values[*value].fmt(f),
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
    let mut hierarchy = parse_line("[[[[[9,8],1],2],3],4]");
    reduce(&mut hierarchy);
    println!("{}", hierarchy);



    // let hierarchies = input.lines().map(parse_line);

    // for hierarchy in hierarchies {
    //     println!("{}", hierarchy);
    // }

    write!(output, "{}", 0).unwrap();
}


pub fn solve_b(input: &String, output: &mut String) {
    write!(output, "{}", 0).unwrap();
}

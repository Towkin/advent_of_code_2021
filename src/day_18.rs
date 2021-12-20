use std::fmt::{Write, Display};
use std::ops::Add;
use std::rc::Rc;

#[derive(Clone)]
enum Element {
    Pair(Box<Pair>),
    Value(u8),
}

#[derive(Clone)]
struct Pair {
    left: Element,
    right: Element,
}

fn make_pair(left: u8, right: u8) -> Pair {
    Pair {
        left: Element::Value(left),
        right: Element::Value(right),
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair {
            left: Element::Pair(Box::new(self)),
            right: Element::Pair(Box::new(rhs)),
        }
    }
}

impl Add<u8> for Pair {
    type Output = Pair;

    fn add(self, rhs: u8) -> Self::Output {
        Pair {
            left: Element::Pair(Box::new(self)),
            right: Element::Value(rhs),
        }
    }
}

impl Add<Pair> for u8 {
    type Output = Pair;

    fn add(self, rhs: Pair) -> Self::Output {
        Pair {
            left: Element::Value(self),
            right: Element::Pair(Box::new(rhs)),
        }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        match &self.left {
            Element::Pair(pair) => pair.fmt(f),
            Element::Value(value) => value.fmt(f),
        }?;
        f.write_char(',')?;
        match &self.right {
            Element::Pair(pair) => pair.fmt(f),
            Element::Value(value) => value.fmt(f),
        }?;
        f.write_char(']')
    }
}

enum Side {
    Left,
    Right,
}

fn parse_line(line: &str) -> Pair {
    let mut pair_stack: Vec<(Pair, Side)> = Vec::new();

    const PAIR_START: u8 = '[' as u8;
    const PAIR_END: u8 = ']' as u8;
    const PAIR_SEPARATOR: u8 = ',' as u8;

    for c in line.as_bytes().iter() {
        match *c {
            PAIR_START => {
                pair_stack.push((Pair {
                    left: Element::Value(0),
                    right: Element::Value(0),
                }, Side::Left));
            },
            PAIR_END => {
                let (closed_pair, _) = pair_stack.pop().unwrap();
                if let Some((top_pair, side)) = pair_stack.last_mut() {
                    let element = Element::Pair(Box::new(closed_pair));
                    match side {
                        Side::Left => top_pair.left = element,
                        Side::Right => top_pair.right = element,
                    }
                } else {
                    return closed_pair;
                }
            },
            PAIR_SEPARATOR => {
                pair_stack.last_mut().unwrap().1 = Side::Right;
            },
            _ => {
                let value = Element::Value(*c - '0' as u8);
                let (pair, side) = pair_stack.last_mut().unwrap();
                match side {
                    Side::Left => pair.left = value,
                    Side::Right => pair.right = value,
                }
            },
        };
    }

    pair_stack.pop().unwrap().0
}

pub fn solve_a(input: &String, output: &mut String) {
    let pairs = input.lines().map(parse_line);

    for pair in pairs {
        println!("{}", pair);
    }

    write!(output, "{}", 0).unwrap();
}


pub fn solve_b(input: &String, output: &mut String) {
    write!(output, "{}", 0).unwrap();
}

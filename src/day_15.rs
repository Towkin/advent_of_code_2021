use std::io::Write;

use pathfinding::prelude;

struct NeighborIterator {
    step: u8,
    origin: (u32, u32),
    max: (u32, u32),
}

impl Iterator for NeighborIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 {
            self.step += 1;
            if self.origin.0 > 0 {
                return Some((self.origin.0 - 1, self.origin.1));
            }
        }
        if self.step == 1 {
            self.step += 1;
            if self.origin.1 > 0 {
                return Some((self.origin.0, self.origin.1 - 1));
            }
        }
        if self.step == 2 {
            self.step += 1;
            if self.origin.0 < self.max.0 {
                return Some((self.origin.0 + 1, self.origin.1));
            }
        }
        if self.step == 3 {
            self.step += 1;
            if self.origin.1 < self.max.1 {
                return Some((self.origin.0, self.origin.1 + 1));
            }
        }

        None
    }
}
fn get_neighbors(pos: &(u32, u32), max: (u32, u32)) -> NeighborIterator {
    NeighborIterator {
        step: 0,
        origin: *pos,
        max,
    }
}

fn solve(size: u32, board: &[u8]) -> u32 {
    let get_index = |pos: (u32, u32)| -> usize {
        (pos.1 * size + pos.0) as usize
    };

    let start: (u32, u32) = (0, 0);
    let end: (u32, u32) = (size - 1, size - 1);
    let path = prelude::dijkstra(&start,
        |pos| get_neighbors(pos, end).map(|pos|
            (pos, board[get_index(pos)] as u32)),
        // |pos| {
        //     board[get_index(*pos)] as u32 + end.0 - pos.0 + end.1 - pos.1
        // },
        |(x, y)| *x == end.0 && *y == end.1).unwrap();

    assert_eq!(path.1, path.0.iter().skip(1).map(|p| board[get_index(*p)] as u32).sum());

    path.1
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut board: [u8; 100 * 100] = [0; 100 * 100];
    for (y, line) in input.lines().enumerate() {
        for x in 0..100 {
            board[y * 100 + x] = line.as_bytes()[x] - '0' as u8;
        }
    }
    write!(output, "{}", solve(100, &board)).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let mut board: Box<[u8; 500 * 500]> = Box::new([0; 500 * 500]);
    for y_tile in 0..5 {
        for (y, line) in input.lines().enumerate() {
            let y = y_tile * 100 + y;
            for x_tile in 0..5 {
                let difficulty_add = (y_tile + x_tile) as u8;
                for x in 0..100 {
                    board[y * 500 + x_tile * 100 + x] = ((line.as_bytes()[x] - '1' as u8 + difficulty_add) % 9) + 1;
                }
            }
        }
    }
    write!(output, "{}", solve(500, board.as_ref())).unwrap();
}

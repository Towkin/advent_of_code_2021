use std::cmp::{max, min};
use std::fmt::Write;

fn flash(energy_levels: &mut [u8; 10*10], x: usize, y: usize) -> u32 {
    let mut flashes = 1;
    energy_levels[y * 10 + x] = 0;
    for y in (max(y as i32 - 1, 0) as usize)..min(y + 2, 10) {
        for x in (max(x as i32 - 1, 0) as usize)..min(x + 2, 10) {
            match energy_levels[y * 10 + x] {
                0 => continue,
                9 => flashes += flash(energy_levels, x, y),
                _ => energy_levels[y * 10 + x] += 1,
            }
        }
    }

    energy_levels[y * 10 + x] = 0;
    flashes
}

pub fn solve_a(input: &String, output: &mut String) {
    let mut energy_levels = [0; 10 * 10];
    input.lines().enumerate().for_each(|(y, line)| {
        for (x, level) in line.as_bytes().iter().enumerate() {
            energy_levels[y * 10 + x] = *level - '0' as u8;
        }
    });

    let mut flashes = 0;
    for _ in 0..100 {
        for y in 0..10 {
            for x in 0..10 {
                energy_levels[y * 10 + x] += 1;
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                if energy_levels[y * 10 + x] > 9 {
                    flashes += flash(&mut energy_levels, x, y);
                }
            }
        }
    }

    write!(output, "{}", flashes).unwrap();
}

pub fn solve_b(input: &String, output: &mut String) {
    let mut energy_levels = [0; 10 * 10];
    input.lines().enumerate().for_each(|(y, line)| {
        for (x, level) in line.as_bytes().iter().enumerate() {
            energy_levels[y * 10 + x] = *level - '0' as u8;
        }
    });

    let mut i = 0;
    loop {
        i += 1;
        for y in 0..10 {
            for x in 0..10 {
                energy_levels[y * 10 + x] += 1;
            }
        }

        let mut flashes = 0;
        for y in 0..10 {
            for x in 0..10 {
                if energy_levels[y * 10 + x] > 9 {
                    flashes += flash(&mut energy_levels, x, y);
                }
            }
        }

        if flashes == 100 {
            break;
        }
    }

    write!(output, "{}", i).unwrap();
}

use std::collections::HashMap;

use self::DigitDisplay::DIGIT_COUNT;

pub fn solve_day_8a(lines: impl Iterator<Item = String>) -> u32 {
    // 1: 2; 3: 4, 7: 3; 8: 7
    let known_digit_lengths: [usize; 4] = [2, 4, 3, 7];

    lines
        .map(|line| line
            .split_once('|')
                .unwrap()
                .1
            .split(' ')
            .filter(|word| known_digit_lengths
                .contains(&word.len())
            )
            .count() as u32
        )
        .sum()
}

mod DigitDisplay
{
    pub const DIGIT_COUNT: usize = 10;

    pub const A: u8 = 1 << 0;
    pub const B: u8 = 1 << 1;
    pub const C: u8 = 1 << 2;
    pub const D: u8 = 1 << 3;
    pub const E: u8 = 1 << 4;
    pub const F: u8 = 1 << 5;
    pub const G: u8 = 1 << 6;

    pub const DIGITS: [u8; DIGIT_COUNT] = [
        A | B | C | E | F | G,
        C | F,
        A | C | D | E | G,
        A | C | D | F | G,
        B | C | D | F,
        A | B | D | F | G,
        A | B | D | E | F | G,
        A | C | F,
        A | B | C | D | E | F | G,
        A | B | C | D | F | G,
    ];

    pub fn get_digit_map() -> [u8; DIGIT_COUNT * DIGIT_COUNT] {
        let mut map: [u8; DIGIT_COUNT * DIGIT_COUNT] = [0; DIGIT_COUNT * DIGIT_COUNT];
        for i in 0..DIGIT_COUNT {
            for j in 0..DIGIT_COUNT {
                map[i * DIGIT_COUNT + j] = DIGITS[i] & DIGITS[j];
            }
        }
        map
    }
}

pub fn solve_day_8b(lines: impl Iterator<Item = String>) -> u32 {
    let digit_map = DigitDisplay::get_digit_map();
    lines
        .map(|line| {
            let (ciphers, values) = line.split_once('|').unwrap();
            let ciphers: Vec<&str> = ciphers.split(' ').collect();
            let mut cipher_map: HashMap<&str, u8> = HashMap::new();
            // TODO [Emanuel Strömgren, 2021-12-08]: Resolve Cipher...
            // Return 0 in the meantime:
            return 0;

            values.split(' ').fold(0, |value, digits| value * 10 + (cipher_map[digits] as u32))
        })
        .sum()
}

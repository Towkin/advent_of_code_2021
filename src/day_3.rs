use std::io::Write;

const LINE_LENGTH: usize = 12;
const CHAR_ZERO: u8 = '0' as u8;
const CHAR_ONE: u8 = '1' as u8;

fn get_binary_frequency<'a>(lines: impl Iterator<Item = &'a str>) -> [i32; LINE_LENGTH] {
    let mut values = [0; LINE_LENGTH];

    for line in lines {
        for (i, c) in line.as_bytes().iter().enumerate() {
            values[i] += match *c {
                CHAR_ZERO => -1,
                CHAR_ONE => 1,
                _ => 0
            };
        }
    }

    values
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let values = get_binary_frequency(lines);

    let mut gamma: u32 = 0;
    for (i, v) in values.iter().enumerate() {
        gamma += (if *v >= 0 { 1 } else { 0 }) << LINE_LENGTH - i - 1;
    }

    let epsilon = !gamma & (u32::MAX >> 32 - LINE_LENGTH);
    write!(output, "{}", gamma * epsilon).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let mut most_common: Vec<&str> = lines.collect();
    let mut least_common = most_common.clone();

    let oxygen_rating = {
        for bit_pos in 0..LINE_LENGTH {
            let most_common_char = if most_common
                .iter()
                .map(|line| match line.as_bytes()[bit_pos] {
                    CHAR_ZERO => -1,
                    CHAR_ONE => 1,
                    _ => 0,
                })
                .sum::<i32>() >= 0 { '1' } else { '0' } as u8;

            most_common.retain(|line| line.as_bytes()[bit_pos] == most_common_char);
            if most_common.len() == 1 {
                break;
            }
        }

        u32::from_str_radix(most_common[0], 2).unwrap()
    };

    let co2_rating = {
        for bit_pos in 0..LINE_LENGTH {
            let most_common_char = if least_common
                .iter()
                .map(|line| match line.as_bytes()[bit_pos] {
                    CHAR_ZERO => -1,
                    CHAR_ONE => 1,
                    _ => 0,
                })
                .sum::<i32>() >= 0 { '1' } else { '0' } as u8;

            least_common.retain(|line| line.as_bytes()[bit_pos] != most_common_char);
            if least_common.len() == 1 {
                break;
            }
        }

        u32::from_str_radix(least_common[0], 2).unwrap()
    };

    write!(output, "{}", oxygen_rating * co2_rating).unwrap();
}

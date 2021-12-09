const LINE_LENGTH: usize = 12;

fn get_binary_frequency(lines: impl Iterator<Item = String>) -> [i32; LINE_LENGTH] {
    let mut values = [0; LINE_LENGTH];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            values[i] += match c {
                '0' => -1,
                '1' => 1,
                _ => 0
            };
        }
    }

    values
}

pub fn solve_day_3a(lines: impl Iterator<Item = String>) -> u32 {
    let values = get_binary_frequency(lines);

    let mut gamma: u32 = 0;
    for (i, v) in values.iter().enumerate() {
        gamma += (if *v >= 0 { 1 } else { 0 }) << LINE_LENGTH - i - 1;
    }

    let epsilon = !gamma & (u32::MAX >> 32 - LINE_LENGTH);
    gamma * epsilon
}

pub fn solve_day_3b(lines: impl Iterator<Item = String>) -> u32 {
    let mut most_common: Vec<String> = lines.collect();
    let mut least_common = most_common.clone();

    for bit_pos in 0..LINE_LENGTH {
        let most_common_char = if most_common
            .iter()
            .map(|line| match line.chars().nth(bit_pos).unwrap() {
                '0' => -1,
                '1' => 1,
                _ => 0,
            })
            .sum::<i32>() >= 0 { '1' } else { '0' } as u8;

        most_common.retain(|line| line.as_bytes()[bit_pos] == most_common_char);
        if most_common.len() == 1 {
            break;
        }
    }

    for bit_pos in 0..LINE_LENGTH {
        let most_common_char = if least_common
            .iter()
            .map(|line| match line.chars().nth(bit_pos).unwrap() {
                '0' => -1,
                '1' => 1,
                _ => 0,
            })
            .sum::<i32>() >= 0 { '1' } else { '0' } as u8;

        least_common.retain(|line| line.as_bytes()[bit_pos] != most_common_char);
        if least_common.len() == 1 {
            break;
        }
    }

    let most_common_line = &most_common[0];
    let least_common_line = &least_common[0];

    let oxygen_rating = u32::from_str_radix(most_common_line.as_str(), 2).unwrap();
    let co2_rating = u32::from_str_radix(least_common_line.as_str(), 2).unwrap();

    oxygen_rating * co2_rating
}

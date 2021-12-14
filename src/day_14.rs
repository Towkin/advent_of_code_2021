use std::{fmt::Write, collections::HashMap};

pub fn solve_a(input: &String, output: &mut String) {
    let mut input = input.lines();
    let mut read_text = input.next().unwrap().to_string();

    let insertion_set: HashMap<&str, char> = HashMap::from_iter(
        input.skip(1).map(|line | {
            let mut line = line.split(" -> ");
            let slice_match = line.next().unwrap();
            let insert_char = line.next().unwrap().chars().next().unwrap();

            (slice_match, insert_char)
        })
    );

    {
        let mut write_text = String::new();
        for _ in 0..10 {
            for i in 0..read_text.len()-1 {
                write_text.write_char(read_text.as_bytes()[i] as char).unwrap();
                if let Some(insert_char) = insertion_set.get(&read_text[i..i+2]) {
                    write_text.write_char(*insert_char).unwrap();
                }
            }
            write_text.write_char(read_text.as_bytes()[read_text.len() - 1] as char).unwrap();
            read_text.clear();
            read_text.write_str(write_text.as_str()).unwrap();
            write_text.clear();
        }
    }

    const OFFSET: usize = 'A' as usize;
    const SIZE: usize = ('Z' as u8 - 'A' as u8) as usize;
    let mut frequency_table: [u32; SIZE] = [0; SIZE];
    for byte in read_text.as_bytes() {
        frequency_table[*byte as usize - OFFSET] += 1;
    }

    let max_count = frequency_table.iter().max().unwrap();
    let min_count = frequency_table.iter().filter(|v| **v > 0).min().unwrap();

    write!(output, "{}", max_count - min_count).unwrap();
}

const SIZE: usize = ('Z' as u8 - 'A' as u8) as usize;
const OFFSET: u8 = 'A' as u8;
type PairLookup = [u8; SIZE * SIZE];
type CountTable = [u64; SIZE];

fn step(pair: (u8, u8), counts: &mut CountTable, lookup: &PairLookup, further_depth: u32) {
    if further_depth == 0 {
        return;
    }

    let left_index = (pair.0 - OFFSET) as usize;
    let right_index = (pair.1 - OFFSET) as usize;
    let center_char = lookup[left_index * SIZE + right_index];
    if center_char == 0 {
        return;
    }
    let center_index = (center_char - OFFSET) as usize;
    counts[center_index] += 1;

    step((pair.0, center_char), counts, lookup, further_depth - 1);
    step((center_char, pair.1), counts, lookup, further_depth - 1);
}

pub fn solve_b(input: &String, output: &mut String) {
    let mut input = input.lines();
    let start_config = input.next().unwrap().as_bytes();

    let lookup = {
        let mut lookup: PairLookup = [0; SIZE * SIZE];
        for line in input.skip(1) {
            let mut line = line.split(" -> ");
            let slice_match = line.next().unwrap().as_bytes();
            let insert_char = line.next().unwrap().as_bytes()[0];

            let left_index = (slice_match[0] - OFFSET) as usize;
            let right_index = (slice_match[1] - OFFSET) as usize;

            lookup[left_index * SIZE + right_index] = insert_char;
        }

        lookup
    };

    let mut counts: CountTable = [0; SIZE];

    for i in 0..(start_config.len() - 1) {
        counts[(start_config[i] - OFFSET) as usize] += 1;
        let pair = (start_config[i], start_config[i + 1]);
        step(pair, &mut counts, &lookup, 40);
    }
    counts[(start_config[start_config.len() - 1] - OFFSET) as usize] += 1;

    let max_count = counts.iter().max().unwrap();
    let min_count = counts.iter().filter(|v| **v > 0).min().unwrap();

    write!(output, "{}", max_count - min_count).unwrap();
}

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

pub fn solve_b(input: &String, output: &mut String) {
}

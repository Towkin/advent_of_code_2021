use std::{fmt::Write, collections::HashMap};

const SIZE: usize = ('Z' as u8 - 'A' as u8) as usize;
const OFFSET: u8 = 'A' as u8;
type CountTable = [u64; SIZE];

fn get_bucket_id(slice: &[u8]) -> u16 {
    ((slice[0] as u16) << 8) | (slice[1] as u16)
}

struct Bucket {
    left: u16,
    right: u16,
    char: u8,
}

fn solve(input: &String, output: &mut String, count: u32) {
    let start_config = input.lines().next().unwrap().as_bytes();

    let buckets: HashMap<u16, Bucket> = HashMap::from_iter(
        input.lines().skip(2).map(|line| {
            let mut line = line.split(" -> ");
            let slice_match = line.next().unwrap().as_bytes();
            let insert_char = line.next().unwrap().as_bytes()[0];

            (get_bucket_id(slice_match), Bucket {
                left: get_bucket_id(&[slice_match[0], insert_char]),
                right: get_bucket_id(&[insert_char, slice_match[1]]),
                char: slice_match[0]
            })
        })
    );

    let mut bucket_counts: HashMap<u16, u64> = HashMap::from_iter(
        buckets.keys().map(|v| (*v, 0)));

    for i in 0..(start_config.len() - 1) {
        bucket_counts.entry(get_bucket_id(&start_config[i..i+2])).and_modify(|v| *v += 1);
    }
    let mut bucket_counts_next = bucket_counts.clone();

    for _ in 0..count {
        for (key, bucket) in buckets.iter() {
            let count = bucket_counts[&key];
            bucket_counts_next.entry(*key).and_modify(|v| *v -= count);
            bucket_counts_next.entry(bucket.left).and_modify(|v| *v += count);
            bucket_counts_next.entry(bucket.right).and_modify(|v| *v += count);
        }

        for (key, count) in bucket_counts_next.iter() {
            bucket_counts.entry(*key).and_modify(|v| *v = *count);
        }
    }

    let mut counts: CountTable = [0; SIZE];
    for (key, bucket) in buckets {
        let count = bucket_counts[&key];
        counts[(bucket.char - OFFSET) as usize] += count;
    }

    counts[(start_config[start_config.len() - 1] - OFFSET) as usize] += 1;

    let max_count = counts.iter().max().unwrap();
    let min_count = counts.iter().filter(|v| **v > 0).min().unwrap();

    write!(output, "{}", max_count - min_count).unwrap();
}


pub fn solve_a(input: &String, output: &mut String) {
    solve(input, output, 10);
}

pub fn solve_b(input: &String, output: &mut String) {
    solve(input, output, 40);
}

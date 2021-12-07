use std::time::Instant;
use std::{env, io};
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
    let (iterations, days) = read_args();

    let mut path = env::current_dir().unwrap();
    path.push("input");

    let time = Instant::now();
    for _ in 0..iterations {
        println!("Solving");
        for day in days.iter() {
            path.push(format!("{}.txt", day));
            {
                let a_input = read_lines(&path);
                let b_input = read_lines(&path);

                let (a, b) = match day {
                    1 => (day_1::solve_day_1a(a_input), day_1::solve_day_1b(b_input) as u64),
                    2 => (day_2::solve_day_2a(a_input), day_2::solve_day_2b(b_input) as u64),
                    3 => (day_3::solve_day_3a(a_input), day_3::solve_day_3b(b_input) as u64),
                    4 => (day_4::solve_day_4a(a_input), day_4::solve_day_4b(b_input) as u64),
                    5 => (day_5::solve_day_5a(a_input), day_5::solve_day_5b(b_input) as u64),
                    6 => (day_6::solve_day_6a(a_input), day_6::solve_day_6b(b_input)),
                    7 => (day_7::solve_day_7a(a_input), day_7::solve_day_7b(b_input) as u64),
                    _ => (0, 0),
                };
                println!("{day}a: {a}\n{day}b: {b}", day = day, a = a, b = b);
            }
            path.pop();
        }
        println!("Done");
    }
    let duration = time.elapsed();
    println!("Completed all iterations in {:?}, average of {:?} per iteration.", duration, duration / iterations);
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
    where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
}

fn read_args() -> (u32, Vec<u32>) {
    let mut args = env::args();

    // Skip first argument
    args.next();

    (
        args.next().unwrap().parse().expect("Bad iteration count"),
        args.map(|a| a.parse().expect("Bad day")).collect(),
    )
}

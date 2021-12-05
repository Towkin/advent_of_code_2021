use std::{env, io};
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let (iterations, days) = read_args();

    let mut path = env::current_dir().unwrap();
    path.push("input");

    for _ in 0..iterations {
        for day in days.iter() {
            path.push(format!("{}.txt", day));
            let (a, b) = match day {
                1 => (day_1::solve_day_1a(read_lines(&path)), day_1::solve_day_1b(read_lines(&path))),
                2 => (day_2::solve_day_2a(read_lines(&path)), day_2::solve_day_2b(read_lines(&path))),
                3 => (day_3::solve_day_3a(read_lines(&path)), day_3::solve_day_3b(read_lines(&path))),
                4 => (day_4::solve_day_4a(read_lines(&path)), day_4::solve_day_4b(read_lines(&path))),
                5 => (day_5::solve_day_5a(read_lines(&path)), day_5::solve_day_5b(read_lines(&path))),
                _ => (0, 0),
            };
            path.pop();
            println!("{}, {}", a,  b);
        }
    }
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

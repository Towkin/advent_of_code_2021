use std::{env, io};
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

mod day_1;

fn main() {
    let (_, day) = read_args();

    let mut path = env::current_dir().unwrap();
    path.push("input");
    path.push(format!("{}.txt", day));

    let lines = read_lines(&path).expect(
        format!("Couldn't read file! {}", path.to_str().unwrap()).as_str());
    let lines = lines.map(|l| {
        l.expect("Bad line!")
    });

    let result_a = day_1::solve_day_1a(lines);

    println!("{}", result_a);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_args() -> (u32, u32) {
    let mut args = env::args();

    // Skip first argument
    args.next();

    (
        args.next().unwrap().parse().expect("Bad iteration count"),
        args.next().unwrap().parse().expect("Bad day")
    )
}

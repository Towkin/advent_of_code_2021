// use std::io::Write;
use std::fmt::Write;
use std::time::Instant;
use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;

fn main() {
    let (iterations, days) = read_args();

    let mut path = env::current_dir().unwrap();
    path.push("input");

    let mut output = std::io::stdout();
    let mut output_text = String::from("Solving\n");
    let mut input_file = String::new();
    let time = Instant::now();
    for _ in 0..iterations {
        output_text.truncate("Solving\n".len());

        for day in days.iter() {
            input_file.clear();
            write!(input_file, "{}.txt", day).unwrap();
            path.push(input_file.as_str());
            {
                let mut input = std::fs::read_to_string(&path).unwrap();
                input.truncate(input.trim_end().len());

                let (a, b) = match day {
                    1 => (day_1::solve_day_1a(&input), day_1::solve_day_1b(&input) as u64),
                    2 => (day_2::solve_day_2a(&input), day_2::solve_day_2b(&input) as u64),
                    3 => (day_3::solve_day_3a(&input), day_3::solve_day_3b(&input) as u64),
                    4 => (day_4::solve_day_4a(&input), day_4::solve_day_4b(&input) as u64),
                    5 => (day_5::solve_day_5a(&input), day_5::solve_day_5b(&input) as u64),
                    6 => (day_6::solve_day_6a(&input), day_6::solve_day_6b(&input)),
                    7 => (day_7::solve_day_7a(&input), day_7::solve_day_7b(&input) as u64),
                    8 => (day_8::solve_day_8a(&input), day_8::solve_day_8b(&input) as u64),
                    9 => (day_9::solve_day_9a(&input), day_9::solve_day_9b(&input) as u64),
                    10 => (day_10::solve_day_10a(&input), day_10::solve_day_10b(&input)),
                    11 => (day_11::solve_day_11a(&input), day_11::solve_day_11b(&input) as u64),
                    _ => (0, 0),
                };

                write!(output_text, "{}a: {}\n{}b: {}\n", day, a, day, b).unwrap();
            }
            path.pop();
        }
        output_text.push_str("Done\n");
        std::io::Write::write(&mut output, output_text.as_bytes()).unwrap();
    }
    let duration = time.elapsed();
    println!("Completed all iterations in {:?}, average of {:?} per iteration.", duration, duration / iterations);
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

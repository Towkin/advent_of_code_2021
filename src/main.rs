use std::fmt::Write;
use std::io::Read;
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
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;

fn main() {
    let (iterations, days) = read_args();

    let mut path = env::current_dir().unwrap();
    path.push("input");

    let mut output = std::io::stdout();
    let mut output_text = String::from("Solving\n");
    let mut input_file = String::new();
    let mut input = String::new();
    let time = Instant::now();
    for _ in 0..iterations {
        output_text.truncate("Solving\n".len());

        for day in days.iter() {
            input.clear();
            write!(input_file, "{}.txt", day).unwrap();
            path.push(input_file.as_str());
            std::fs::File::open(&path).unwrap().read_to_string(&mut input).unwrap();
            path.pop();
            input_file.clear();

            input.truncate(input.trim_end().len());
            solve(*day, &input, &mut output_text);
        }
        output_text.push_str("Done\n");
        std::io::Write::write(&mut output, output_text.as_bytes()).unwrap();
    }
    let duration = time.elapsed();
    println!("Completed all iterations in {:?}, average of {:?} per iteration.", duration, duration / iterations);
}

macro_rules! solve_and_print_day {
    ($day_module:ident, $day:expr, $input:expr, $output:expr) => {
        {
            write!($output, "{}a: ", $day).unwrap();
            $day_module::solve_a($input, $output);
            write!($output, "\n{}b: ", $day).unwrap();
            $day_module::solve_b($input, $output);
            write!($output, "\n").unwrap();
        }
    };
}

fn solve(day: u32, input: &String, output: &mut String) {
    match day {
        1 => solve_and_print_day!(day_1, 1, input, output),
        2 => solve_and_print_day!(day_2, 2, input, output),
        3 => solve_and_print_day!(day_3, 3, input, output),
        4 => solve_and_print_day!(day_4, 4, input, output),
        5 => solve_and_print_day!(day_5, 5, input, output),
        6 => solve_and_print_day!(day_6, 6, input, output),
        7 => solve_and_print_day!(day_7, 7, input, output),
        8 => solve_and_print_day!(day_8, 8, input, output),
        9 => solve_and_print_day!(day_9, 9, input, output),
        10 => solve_and_print_day!(day_10, 10, input, output),
        11 => solve_and_print_day!(day_11, 11, input, output),
        12 => solve_and_print_day!(day_12, 12, input, output),
        13 => solve_and_print_day!(day_13, 13, input, output),
        14 => solve_and_print_day!(day_14, 14, input, output),
        15 => solve_and_print_day!(day_15, 15, input, output),
        16 => solve_and_print_day!(day_16, 16, input, output),
        17 => solve_and_print_day!(day_17, 17, input, output),
        18 => solve_and_print_day!(day_18, 18, input, output),
        _ => panic!("Could not solve day {}", day),
    };
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

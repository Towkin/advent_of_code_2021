use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
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
mod day_19;
mod day_20;
mod day_21;

fn main() {
    let (iterations, days) = read_args();

    let mut path = env::current_dir().unwrap();
    path.push("input");

    let output = std::io::stdout();
    let mut writer = std::io::BufWriter::with_capacity(
        64 * 1024, 
        output.lock());
    let mut input_file = String::new();
    let mut input = String::new();
    let time = Instant::now();
    for _ in 0..iterations {
        write!(&mut writer, "Solving\n").unwrap();

        for day in days.iter() {
            input.clear();
            write!(&mut input_file, "{}.txt", day).unwrap();
            path.push(input_file.as_str());
            File::open(&path).unwrap().read_to_string(&mut input).unwrap();
            path.pop();
            input_file.clear();

            input.truncate(input.trim_end().len());
            solve(*day, &input, &mut writer);
        }
        write!(&mut writer, "Done\n").unwrap();
    }
    let duration = time.elapsed();
    write!(&mut writer, "Completed all iterations in {:?}, average of {:?} per iteration.", duration, duration / iterations).unwrap();
    std::io::Write::flush(&mut writer).unwrap();
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

fn solve(day: u32, input: &String, output: &mut impl Write) {
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
        19 => solve_and_print_day!(day_19, 19, input, output),
        20 => solve_and_print_day!(day_20, 20, input, output),
        21 => solve_and_print_day!(day_21, 21, input, output),
        _ => panic!("Could not solve day {}", day),
    };
}

const MIN_DAY: u32 = 1;
const MAX_DAY: u32 = 21;

fn read_args() -> (u32, Vec<u32>) {
    let mut args = env::args();

    // Skip first argument
    args.next();

    let iterations = if let Some(iterations) = args.next() { iterations.parse().expect("Bad iteration count") } else { 1 };
    let mut days: Vec<u32> = args.map(|a| {
        let day = a.parse().map_err(|_| "Could not parse day")?;
        if day < MIN_DAY || day > MAX_DAY {
            Err("Day not in bounds")
        } else {
            Ok(day)
        }
    }).map(|day| day.expect("Bad day")).collect();
    if days.len() == 0 {
        for i in MIN_DAY..MAX_DAY + 1 {
            days.push(i);
        }
    }

    ( iterations, days )
}

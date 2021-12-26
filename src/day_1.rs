use std::io::Write;

pub fn solve_a(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let mut numbers = lines
        .filter_map(|l| l.parse::<i32>().ok());
    let mut previous = numbers.next().unwrap();

    let mut count = 0;
    for value in numbers {
        if previous < value {
            count += 1;
        }
        previous = value;
    }

    write!(output, "{}", count).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let numbers: Vec<i32> = lines
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();
    let mut sums = numbers.windows(3).map(|i| i.iter().sum());
    let mut previous: i32 = sums.next().unwrap();

    let mut count = 0;
    for value in sums {
        if previous < value {
            count += 1;
        }
        previous = value;
    }

    write!(output, "{}", count).unwrap();
}

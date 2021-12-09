
pub fn solve_day_1a(input: &String) -> u32 {
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
    count
}

pub fn solve_day_1b(input: &String) -> u32 {
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
    count
}

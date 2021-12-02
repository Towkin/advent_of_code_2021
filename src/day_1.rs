
pub fn solve_day_1a(lines: impl Iterator<Item = String>) -> u32 {
    let mut numbers = lines.map(|l| l.parse::<i32>().unwrap());
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

pub fn solve_day_1b(lines: impl Iterator<Item = String>) -> u32 {
    let mut numbers = lines.map(|l| l.parse::<i32>().unwrap());

    0
}

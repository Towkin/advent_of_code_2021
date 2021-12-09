use std::cmp::min;

pub fn solve_day_7a(input: &String) -> u32 {
    let numbers: Vec<u32> = input.split(',').map(|number| number.parse().unwrap()).collect();

    let range = *numbers.iter().min().unwrap()..*numbers.iter().max().unwrap();
    let mut best = u32::MAX;
    for i in range {
        best = min(best, numbers.iter().map(|n| (*n as i32 - i as i32).abs() as u32).sum());
    }

    best
}

pub fn solve_day_7b(input: &String) -> u32 {
    let numbers: Vec<u32> = input.split(',').map(|number| number.parse().unwrap()).collect();

    let range = *numbers.iter().min().unwrap()..*numbers.iter().max().unwrap()+1;
    let cost: Vec<u32> = range.clone().map(|i| (0..i+1).sum()).collect();
    let mut best = u32::MAX;
    for i in range {
        best = min(best, numbers.iter()
            .map(|n| cost[(*n as i32 - i as i32).abs() as usize]).sum());
    }

    best
}

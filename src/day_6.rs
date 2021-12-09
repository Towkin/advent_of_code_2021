
pub fn solve_day_6a(input: &String) -> u32 {
    let lines = input.lines();
    let lines: Vec<&str> = lines.collect();
    let fish_days = lines[0]
        .split(',')
        .map(|number| number.parse::<usize>().unwrap());

    const MAX_DAYS: usize = 8 + 1;
    let mut population: [u32; MAX_DAYS] = [0; MAX_DAYS];
    for fish_at_day in fish_days {
        population[fish_at_day] += 1;
    }

    for _ in 0..80 {
        let new_parent_population = population[0];

        // Step one day forward.
        for i in 1..MAX_DAYS {
            population[i - 1] = population[i];
        }

        // Push the parent back to day 6.
        population[6] += new_parent_population;

        // Set the new-borns
        population[8] = new_parent_population;
    }

    population.iter().sum()
}

pub fn solve_day_6b(input: &String) -> u64 {
    let lines = input.lines();
    let lines: Vec<&str> = lines.collect();
    let fish_days = lines[0]
        .split(',')
        .map(|number| number.parse::<usize>().unwrap());

    const MAX_DAYS: usize = 8 + 1;
    let mut population: [u64; MAX_DAYS] = [0; MAX_DAYS];
    for fish_at_day in fish_days {
        population[fish_at_day] += 1;
    }

    for _ in 0..256 {
        let new_parent_population = population[0];

        // Step one day forward.
        for i in 1..MAX_DAYS {
            population[i - 1] = population[i];
        }

        // Push the parent back to day 6.
        population[6] += new_parent_population;

        // Set the new-borns
        population[8] = new_parent_population;
    }

    population.iter().sum()
}

use std::fmt::Write;

pub fn solve_a(input: &String, output: &mut String) {
    let mut brace_stack: Vec<char> = Vec::new();
    let sum: u32 = input.lines().filter_map(|line| {
        for brace in line.chars() {
            if let Some(end_brace) = match brace {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '<' => Some('>'),
                _ => None,
            } {
                brace_stack.push(end_brace);
            } else if brace != brace_stack.pop().unwrap() {
                brace_stack.clear();
                return Some(brace);
            }
        }

        brace_stack.clear();
        None
    }).map(|end_brace| match end_brace {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }).sum();

    write!(output, "{}", sum).unwrap();
}

pub fn solve_b(input: &String, output: &mut String) {
    let mut brace_stack: Vec<char> = Vec::new();
    let mut points: Vec<u64> = input.lines().filter_map(|line| {
        for brace in line.chars() {
            if let Some(end_brace) = match brace {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '<' => Some('>'),
                _ => None,
            } {
                brace_stack.push(end_brace);
            } else if brace != brace_stack.pop().unwrap() {
                brace_stack.clear();
                return None;
            }
        }

        let mut line_points = 0;
        for end_brace in brace_stack.iter().rev() {
            let brace_points = match end_brace {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            };
            line_points = (line_points * 5) + brace_points;
        }
        brace_stack.clear();
        Some(line_points)
    }).collect();

    points.sort();
    write!(output, "{}", points[points.len() / 2]).unwrap();
}

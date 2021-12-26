use std::io::Write;

pub fn solve_a(input: &String, output: &mut impl Write) {
    let mut horizontal_position: u32 = 0;
    let mut vertical_position: u32 = 0;

    for line in input.lines() {
        match line.find(' ') {
            Some(break_point) => {
                let (direction, offset) = line.split_at(break_point);
                let offset: u32 = offset.trim().parse().expect(offset);
                match direction {
                    "forward" => horizontal_position += offset,
                    "up" => vertical_position -= offset,
                    "down" => vertical_position += offset,
                    _ => ()
                }
            },
            None => ()
        }
    }

    write!(output, "{}", horizontal_position * vertical_position).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let mut horizontal_position: u32 = 0;
    let mut vertical_position: u32 = 0;
    let mut aim_position: u32 = 0;

    for line in input.lines() {
        match line.find(' ') {
            Some(break_point) => {
                let (direction, offset) = line.split_at(break_point);
                let offset: u32 = offset.trim().parse().expect(offset);
                match direction {
                    "forward" => {
                        horizontal_position += offset;
                        vertical_position += aim_position * offset;
                    },
                    "up" => aim_position -= offset,
                    "down" => aim_position += offset,
                    _ => ()
                }
            },
            None => ()
        }
    }

    write!(output, "{}", horizontal_position * vertical_position).unwrap();
}

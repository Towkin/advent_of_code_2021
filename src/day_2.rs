pub fn solve_day_2a(lines: impl Iterator<Item = String>) -> i32 {
    let mut horizontal_position: i32 = 0;
    let mut vertical_position: i32 = 0;

    for line in lines {
        match line.find(' ') {
            Some(break_point) => {
                let (direction, offset) = line.split_at(break_point);
                let offset: i32 = offset.trim().parse().expect(offset);
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

    horizontal_position * vertical_position
}

pub fn solve_day_2b(lines: impl Iterator<Item = String>) -> i32 {
    let mut horizontal_position: i32 = 0;
    let mut vertical_position: i32 = 0;
    let mut aim_position: i32 = 0;

    for line in lines {
        match line.find(' ') {
            Some(break_point) => {
                let (direction, offset) = line.split_at(break_point);
                let offset: i32 = offset.trim().parse().expect(offset);
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

    horizontal_position * vertical_position
}

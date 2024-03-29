
use std::{collections::HashSet, fmt::Display, io::Write};

const SIZE: usize = 5;

struct BingoBoard {
    values: [u32; SIZE * SIZE],
}

impl BingoBoard {
    fn row_won(&self, row: usize, winning_numbers: &HashSet<u32>) -> bool {
        for i in 0..SIZE {
            if !winning_numbers.contains(&self.values[row * SIZE + i]) {
                return false;
            }
        }
        true
    }

    fn column_won(&self, column: usize, winning_numbers: &HashSet<u32>) -> bool {
        for i in 0..SIZE {
            if !winning_numbers.contains(&self.values[i * SIZE + column]) {
                return false;
            }
        }
        true
    }

    pub fn board_won(&self, winning_numbers: &HashSet<u32>) -> bool {
        for i in 0..SIZE {
            if self.column_won(i, winning_numbers)
            || self.row_won(i, winning_numbers) {
                return true;
            }
        }
        false
    }

    pub fn parse(lines: &[&str]) -> BingoBoard {
        let mut board = BingoBoard {
            values: [0; SIZE * SIZE],
        };
        for (row, line) in lines.iter().enumerate() {
            let line = line
                .split(' ')
                .filter(|word| !word.is_empty())
                .map(|word| word.trim().parse::<u32>().expect(
                    format!("Failed to parse '{}'", word).as_str()));

            for (column, value) in line.enumerate() {
                board.values[row * SIZE + column] = value;
            }
        }

        board
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..SIZE {
            writeln!(f)?;

            for column in 0..SIZE {
                write!(f, "{:>3}", self.values[row * SIZE + column])?
            }
        };

        Ok(())
    }
}

fn get_boards(lines: &Vec<&str>) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut i = 2;
    while i < lines.len() {
        boards.push(BingoBoard::parse(&lines[i..i+5]));
        i += 6;
    }
    boards
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let lines: Vec<&str> = lines.collect();
    let series = lines[0]
        .split(',')
        .map(|word| word.trim().parse().unwrap());

    let boards = get_boards(&lines);

    let unmarked_number_product = || -> u32 {
        let mut win_table: HashSet<u32> = HashSet::from_iter(series.clone().take(4));
        for number in series.skip(4) {
            win_table.insert(number);

            for board in boards.iter() {
                if board.board_won(&win_table) {
                    return board.values.iter()
                        .filter(|v| !win_table.contains(v))
                        .sum::<u32>() * number
                }
            }
        }

        0
    }();

    write!(output, "{}", unmarked_number_product).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let lines = input.lines();
    let lines: Vec<&str> = lines.collect();

    let series = lines[0].split(',')
        .map(|word| word.trim().parse().unwrap());

    let mut boards = get_boards(&lines);

    let unmarked_number_product = || -> u32 {
        let mut win_table: HashSet<u32> = HashSet::from_iter(series.clone().take(50));
        for number in series.skip(50) {
            win_table.insert(number);
            if boards.len() == 1 {
                if boards[0].board_won(&win_table) {
                    return boards[0].values.iter()
                        .filter(|v| !win_table.contains(v))
                        .sum::<u32>() * number;
                }
            }

            boards.retain(|board| !board.board_won(&win_table));
        }
        0
    }();

    write!(output, "{}", unmarked_number_product).unwrap();
}
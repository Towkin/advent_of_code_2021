use std::fmt::Write;

pub fn solve_a(input: &String, output: &mut String) {
    // 1: 2; 4: 3, 7: 3; 8: 7
    let known_digit_lengths: [usize; 4] = [2, 4, 3, 7];

    let sum: u32 = input.lines()
        .map(|line| line
            .split_once('|')
                .unwrap()
                .1
            .split(' ')
            .filter(|word| known_digit_lengths
                .contains(&word.len())
            )
            .count() as u32
        )
        .sum();

    write!(output, "{}", sum).unwrap();
}

mod digit_signals
{
    pub const A: u8 = 1 << 0;
    pub const B: u8 = 1 << 1;
    pub const C: u8 = 1 << 2;
    pub const D: u8 = 1 << 3;
    pub const E: u8 = 1 << 4;
    pub const F: u8 = 1 << 5;
    pub const G: u8 = 1 << 6;

    pub const DIGITS: [u8; 10] = [
        /* 0 */ A | B | C |     E | F | G,
        /* 1 */         C |         F,
        /* 2 */ A |     C | D | E |     G,
        /* 3 */ A |     C | D |     F | G,
        /* 4 */     B | C | D |     F,
        /* 5 */ A | B |     D |     F | G,
        /* 6 */ A | B |     D | E | F | G,
        /* 7 */ A |     C |         F,
        /* 8 */ A | B | C | D | E | F | G,
        /* 9 */ A | B | C | D |     F | G,
    ];
}

pub fn solve_b(input: &String, output: &mut String) {
    let sum: u32 = input.lines()
        .map(|line| {
            let (ciphers, values) = line.split_once('|').unwrap();
            let ciphers: Vec<&str> = ciphers.split(' ').collect();

            let signal_1 = ciphers.iter().find(|cipher| cipher.len() == 2).unwrap();
            let signal_4 = ciphers.iter().find(|cipher| cipher.len() == 4).unwrap();
            let signal_7 = ciphers.iter().find(|cipher| cipher.len() == 3).unwrap();
            let signal_8 = ciphers.iter().find(|cipher| cipher.len() == 7).unwrap();

            let cf = signal_1;
            let acf = signal_7;
            let a = acf.chars()
                .filter(|s| !signal_1.contains(*s))
                .next().unwrap();
            let bcdf = *signal_4;
            let bd = bcdf.chars()
                .filter(|s| !cf.contains(*s));

            let signal_9 = ciphers.iter()
                .find(|cipher| cipher.len() == 6 && bcdf.chars()
                    .all(|s| cipher.contains(s))).unwrap();

            let signal_6 = ciphers.iter()
                .find(|cipher| cipher.len() == 6
                    && *cipher != signal_9
                    && bd.clone().all(|s| cipher.contains(s))).unwrap();

            let signal_0 = ciphers.iter()
                .find(|cipher| cipher.len() == 6 && *cipher != signal_9
                    && *cipher != signal_6)
                .unwrap();

            let abcdefg = signal_8;
            let abcefg = signal_0;
            let d = abcdefg.chars()
                .find(|s| !abcefg.contains(*s))
                .unwrap();

            let b = bd.clone().find(|s| *s != d).unwrap();
            let abcdfg = signal_9;
            let e = signal_8.chars()
                .find(|s| !abcdfg.contains(*s))
                .unwrap();

            let signal_2 = ciphers.iter()
                .find(|cipher| cipher.len() == 5 && cipher.contains(e))
                .unwrap();

            let acdeg = signal_2;
            let c = cf.chars().find(|s| acdeg.contains(*s)).unwrap();
            let f = cf.chars().find(|s| *s != c).unwrap();

            let acde = [a, c, d, e];
            let g = acdeg.chars().find(|s| !acde.contains(s)).unwrap();

            values.split(' ')
                .filter(|cipher| cipher.len() > 0)
                .map(|cipher| cipher.chars()
                    .fold(0 as u8, |digits, s| {
                        let signal =
                                 if s == a { digit_signals::A }
                            else if s == b { digit_signals::B }
                            else if s == c { digit_signals::C }
                            else if s == d { digit_signals::D }
                            else if s == e { digit_signals::E }
                            else if s == f { digit_signals::F }
                            else if s == g { digit_signals::G }
                            else { panic!("Invalid digit! {}", s) };
                        digits | signal
                    })
                )
                .map(|digit_bits| digit_signals::DIGITS.iter()
                    .position(|d| *d == digit_bits).unwrap() as u32)
                .fold(0, |value, digit| value * 10 + digit)
        })
        .sum();

    write!(output, "{}", sum).unwrap();
}

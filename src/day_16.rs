use std::fmt::Write;

fn bit(bytes: &[u8], index: usize) -> u8 {
    let byte_pos = index / 8;
    let bit_pos = index % 8;
    bytes[byte_pos] >> (7 - bit_pos) & 1
}

fn get_byte(bytes: &[u8], end_bit_index: usize) -> u8 {
    if end_bit_index <= 8 {
        return bytes[0] >> (8 - end_bit_index);
    }

    let byte_pos = end_bit_index / 8;
    let bit_pos = end_bit_index % 8;

    if bit_pos == 0 {
        return bytes[byte_pos - 1];
    }

    let mask_minus_1 = u8::MAX >> bit_pos;
    let mask_0 = !mask_minus_1;

    let byte_minus_1 = bytes[byte_pos - 1] & mask_minus_1;
    let byte_0 = bytes[byte_pos] & mask_0;

    (byte_minus_1 << bit_pos) | (byte_0 >> (8 - bit_pos))
}


/*
01234567_01234567
00000000_00000000
00000011_11111100
      |-_-----|
      6       13

*/

pub fn solve_a(input: &String, output: &mut String) {
    let input = hex::decode(input).unwrap();

    print!("{:08b}", input[0]);
    print!("{:08b}", input[1]);
    println!();
    println!("---");
    for i in 0..16 {
        print!("{}", bit(&input, i));
    }
    println!();
    println!("---");

    println!("{:b}", get_byte(&input, 3));
    println!("  {:08b}", get_byte(&input, 10));
    println!("       {:08b}", get_byte(&input, 15));
    println!("        {:08b}", get_byte(&input, 16));
}

pub fn solve_b(input: &String, output: &mut String) {
    write!(output, "{}", 0).unwrap();
}

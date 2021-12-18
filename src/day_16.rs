use std::fmt::Write;

fn get_bit(bytes: &[u8], index: usize) -> u8 {
    let byte_pos = index / 8;
    let bit_pos = index % 8;
    if bit_pos == 0 {
        bytes[byte_pos - 1] & 1
    } else {
        bytes[byte_pos] >> (8 - bit_pos) & 1
    }
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
    let byte_0 = if byte_pos < bytes.len() { bytes[byte_pos] & mask_0 } else { 0 };

    (byte_minus_1 << bit_pos) | (byte_0 >> (8 - bit_pos))
}

fn sum_packet_versions(bytes: &[u8], bit_offset: &mut usize) -> u32 {
    *bit_offset += 6;
    let packet_info = get_byte(bytes, *bit_offset) & 0b111111;
    let packet_version = packet_info >> 3;
    let packet_type_id = packet_info & 0b111;

    let mut version_sum: u32 = packet_version as u32;
    match packet_type_id {
        // 4 is Literal
        4 => {
            loop {
                *bit_offset += 5;
                let literal_info = get_byte(bytes, *bit_offset) & 0b11111;
                if literal_info >> 4 & 1 == 0 {
                    break;
                }
            }
        },
        // else Operator
        _ => {
            // First bit decides sub packets' bit length
            *bit_offset += 1;

            let sub_packet_type = get_bit(bytes, *bit_offset);

            // Sub packets' bit length
            if sub_packet_type == 0 {
                *bit_offset += 15;
                let major_byte = get_byte(bytes, *bit_offset - 8) & 0b1111111;
                let minor_byte = get_byte(bytes, *bit_offset);
                let sub_packets_bit_count = (major_byte as u16) << 8 | minor_byte as u16;

                let end_offset = *bit_offset + sub_packets_bit_count as usize;
                while *bit_offset < end_offset {
                    version_sum += sum_packet_versions(bytes, bit_offset);
                }
            // Sub packets' count
            } else {
                *bit_offset += 11;
                let major_byte = get_byte(bytes, *bit_offset - 8) & 0b111;
                let minor_byte = get_byte(bytes, *bit_offset);
                let sub_packets_count = (major_byte as u16) << 8 | minor_byte as u16;

                for _ in 0..sub_packets_count {
                    version_sum += sum_packet_versions(bytes, bit_offset);
                }
            }
        }
    }

    version_sum
}

pub fn solve_a(input: &String, output: &mut String) {
    let input = hex::decode(input).unwrap();

    let sum = sum_packet_versions(&input, &mut 0);
    write!(output, "{}", sum).unwrap();
}

fn parse_sub_packets(bytes: &[u8], bit_offset: &mut usize) -> Vec<u64> {
    // First bit decides sub packets' bit length
    *bit_offset += 1;

    let sub_packet_type = get_bit(bytes, *bit_offset);

    let mut result = Vec::new();
    // Sub packets' bit length
    if sub_packet_type == 0 {
        *bit_offset += 15;
        let major_byte = get_byte(bytes, *bit_offset - 8) & 0b1111111;
        let minor_byte = get_byte(bytes, *bit_offset);
        let sub_packets_bit_count = (major_byte as u16) << 8 | minor_byte as u16;

        let end_offset = *bit_offset + sub_packets_bit_count as usize;
        while *bit_offset < end_offset {
            result.push(operate_packets(bytes, bit_offset));
        }
    // Sub packets' count
    } else {
        *bit_offset += 11;
        let major_byte = get_byte(bytes, *bit_offset - 8) & 0b111;
        let minor_byte = get_byte(bytes, *bit_offset);
        let sub_packets_count = (major_byte as u16) << 8 | minor_byte as u16;

        for _ in 0..sub_packets_count {
            result.push(operate_packets(bytes, bit_offset));
        }
    }
    result
}

fn operate_packets(bytes: &[u8], bit_offset: &mut usize) -> u64 {
    *bit_offset += 6;
    let packet_info = get_byte(bytes, *bit_offset) & 0b111111;
    let packet_type_id = packet_info & 0b111;

    match packet_type_id {
        // 4 is Literal
        4 => {
            let mut literal: u64 = 0;
            loop {
                *bit_offset += 5;
                let literal_info = get_byte(bytes, *bit_offset) & 0b11111;
                let literal_value = literal_info & 0b1111;
                literal = literal << 4 | literal_value as u64;
                if literal_info >> 4 & 1 == 0 {
                    break literal
                }
            }
        },
        // Sum
        0 => parse_sub_packets(bytes, bit_offset).iter().sum(),
        // Product
        1 => parse_sub_packets(bytes, bit_offset).iter().fold(1, |f, v| f * *v),
        // Min
        2 => *parse_sub_packets(bytes, bit_offset).iter().min().unwrap(),
        // Max
        3 => *parse_sub_packets(bytes, bit_offset).iter().max().unwrap(),
        // Greater than
        5 => {
            let packets = parse_sub_packets(bytes, bit_offset);
            if packets[0] > packets[1] { 1 } else { 0 }
        },
        // Less than
        6 => {
            let packets = parse_sub_packets(bytes, bit_offset);
            if packets[0] < packets[1] { 1 } else { 0 }
        },
        // Equal to
        7 => {
            let packets = parse_sub_packets(bytes, bit_offset);
            if packets[0] == packets[1] { 1 } else { 0 }
        },
        _ => panic!(),
    }
}

pub fn solve_b(input: &String, output: &mut String) {
    let input = hex::decode(input).unwrap();

    let result = operate_packets(&input, &mut 0);
    write!(output, "{}", result).unwrap();
}

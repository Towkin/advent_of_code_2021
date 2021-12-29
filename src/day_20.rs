use std::io::Write;

fn get_value_map(input_map: &str) -> [bool; 512]
{
    let mut value_map: [bool; 512] = [false; 512];
    for i in 0..512 {
        value_map[i] = match input_map.as_bytes()[i] {
            b'.' => false,
            b'#' => true,
            _ => panic!(),
        };
    }
    value_map
}

fn get_value_index(image: &[bool], size: usize, x: usize, y: usize) -> usize {
        256 * image[(y - 1) * size + x - 1] as usize |
        128 * image[(y - 1) * size + x + 0] as usize |
        64 * image[(y - 1) * size + x + 1] as usize |
        32 * image[(y + 0) * size + x - 1] as usize |
        16 * image[(y + 0) * size + x + 0] as usize |
        8 * image[(y + 0) * size + x + 1] as usize |
        4 * image[(y + 1) * size + x - 1] as usize |
        2 * image[(y + 1) * size + x + 0] as usize |
        1 * image[(y + 1) * size + x + 1] as usize
}

fn enhance(value_map: &[bool; 512], size: usize, inset: usize, image: &[bool], output: &mut [bool]) {
    for y in inset..size-inset {
        for x in inset..size-inset {
            let value_index = get_value_index(image, size, x, y);
            output[y * size + x] = value_map[value_index];
        }
    }
}

fn _print_image(size: usize, image: &[bool]) {
    for y in 0..size {
        for x in 0..size {
            if image[y * size + x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_image<const N: usize>(lines: &str, size: usize, inset: usize) -> [bool; N] {
    let mut image = [false; N];
    let mut lines = lines.lines().map(|l| l.as_bytes());
    for y in inset..size-inset {
        let line = lines.next().unwrap();
        for x in inset..size-inset {
            image[y * size + x] = match line[x - inset] {
                b'.' => false,
                b'#' => true,
                _ => panic!(),
            };
        }
    }
    image
}

pub fn solve_a(input: &String, output: &mut impl Write) {
    let (input_map, input_image) = input.split_once("\n").unwrap();
    let input_image = input_image.trim_start();
    let value_map = get_value_map(input_map);

    const ITERATIONS: usize = 2;
    const BASE_RESOLUTION: usize = 100;
    const SIDE_RESOLUTION: usize = BASE_RESOLUTION + (ITERATIONS + 1) * 2;
    const TOTAL_RESOLUTION: usize = SIDE_RESOLUTION * SIDE_RESOLUTION;
    let mut image = get_image::<TOTAL_RESOLUTION>(
        input_image, SIDE_RESOLUTION, ITERATIONS + 1
    );

    let mut infinite_plane = false;
    for i in 0..ITERATIONS {
        infinite_plane = value_map[infinite_plane as usize * 511];
        let mut output = [infinite_plane; SIDE_RESOLUTION * SIDE_RESOLUTION];
        enhance(&value_map, SIDE_RESOLUTION, ITERATIONS - i, &image, &mut output);
        image.copy_from_slice(&output);
    }

    write!(output, "{}", image.iter().filter(|v| **v).count()).unwrap();
}

pub fn solve_b(input: &String, output: &mut impl Write) {
    let (input_map, input_image) = input.split_once("\n").unwrap();
    let input_image = input_image.trim_start();
    let value_map = get_value_map(input_map);

    const ITERATIONS: usize = 50;
    const BASE_RESOLUTION: usize = 100;
    const SIDE_RESOLUTION: usize = BASE_RESOLUTION + (ITERATIONS + 1) * 2;
    const TOTAL_RESOLUTION: usize = SIDE_RESOLUTION * SIDE_RESOLUTION;
    let mut image = get_image::<TOTAL_RESOLUTION>(
        input_image, SIDE_RESOLUTION, ITERATIONS + 1
    );

    let mut infinite_plane = false;
    for i in 0..ITERATIONS {
        infinite_plane = value_map[infinite_plane as usize * 511];
        let mut output = [infinite_plane; SIDE_RESOLUTION * SIDE_RESOLUTION];
        enhance(&value_map, SIDE_RESOLUTION, ITERATIONS - i, &image, &mut output);
        image.copy_from_slice(&output);
    }

    write!(output, "{}", image.iter().filter(|v| **v).count()).unwrap();
}

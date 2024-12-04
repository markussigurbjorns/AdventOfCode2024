use std::fs::File;
use std::io::prelude::*;
use std::result;

type Result<T> = result::Result<T, ()>;

const XMAS_LENGTH: usize = 4;
const XMAS_BYTES: [u8; 4] = [88, 77, 65, 83];

const MAS_LENGTH: usize = 3;
const MAS_BYTES: [u8; 3] = [77, 65, 83];

fn main() -> Result<()> {
    let mut file = File::open("day04/input.txt").map_err(|err| {
        eprintln!("ERROR: could not open file {err}");
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| {
        eprintln!("ERROR: could not map contents of a file to a string {err}");
    })?;
    solve_part1(&contents);
    solve_part2(&contents);
    Ok(())
}

fn check_right(index: usize, line_length: usize, length: usize) -> bool {
    if line_length - (index % line_length) >= length {
        return true;
    }
    false
}

fn check_left(index: usize, line_length: usize, length: usize) -> bool {
    if (index) % line_length + 1 >= length {
        return true;
    }
    false
}

fn check_up(index: usize, line_length: usize, length: usize) -> bool {
    if (index) / line_length + 1 >= length {
        return true;
    }
    false
}

fn check_down(index: usize, line_length: usize, full_length: usize, length: usize) -> bool {
    if (full_length - index - 1) / line_length + 1 >= length {
        return true;
    }
    false
}

fn solve_part1(input: &String) {
    let mut count = 0;
    let mut flat_input = String::new();
    let mut line_length = 0;
    for line in input.lines() {
        line_length = line.len();
        flat_input.push_str(line);
    }
    let full_length = flat_input.len();
    let flat_input_as_bytes: &[u8] = flat_input.as_bytes();

    for i in 0..flat_input_as_bytes.len() {
        if flat_input_as_bytes[i] != XMAS_BYTES[0] {
            continue;
        }
        // check for boundaries
        let r: bool = check_right(i, line_length, XMAS_LENGTH);
        let l: bool = check_left(i, line_length, XMAS_LENGTH);
        let u: bool = check_up(i, line_length, XMAS_LENGTH);
        let d: bool = check_down(i, line_length, full_length, XMAS_LENGTH);

        //println!(
        //    "index: {} - boundaries right: {}, left: {}, up: {}, down: {} ",
        //    i, r, l, u, d
        //);

        if r {
            if [
                flat_input_as_bytes[i],
                flat_input_as_bytes[i + 1],
                flat_input_as_bytes[i + 2],
                flat_input_as_bytes[i + 3],
            ] == XMAS_BYTES
            {
                count += 1;
            }

            if u {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i - line_length + 1],
                    flat_input_as_bytes[i - 2 * line_length + 2],
                    flat_input_as_bytes[i - 3 * line_length + 3],
                ] == XMAS_BYTES
                {
                    count += 1;
                }
            }
            if d {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i + line_length + 1],
                    flat_input_as_bytes[i + 2 * line_length + 2],
                    flat_input_as_bytes[i + 3 * line_length + 3],
                ] == XMAS_BYTES
                {
                    count += 1;
                }
            }
        }
        if l {
            if [
                flat_input_as_bytes[i],
                flat_input_as_bytes[i - 1],
                flat_input_as_bytes[i - 2],
                flat_input_as_bytes[i - 3],
            ] == XMAS_BYTES
            {
                count += 1;
            }

            if u {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i - line_length - 1],
                    flat_input_as_bytes[i - 2 * line_length - 2],
                    flat_input_as_bytes[i - 3 * line_length - 3],
                ] == XMAS_BYTES
                {
                    count += 1;
                }
            }
            if d {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i + line_length - 1],
                    flat_input_as_bytes[i + 2 * line_length - 2],
                    flat_input_as_bytes[i + 3 * line_length - 3],
                ] == XMAS_BYTES
                {
                    count += 1;
                }
            }
        }
        if u {
            if [
                flat_input_as_bytes[i],
                flat_input_as_bytes[i - line_length],
                flat_input_as_bytes[i - 2 * line_length],
                flat_input_as_bytes[i - 3 * line_length],
            ] == XMAS_BYTES
            {
                count += 1;
            }
        }
        if d {
            if [
                flat_input_as_bytes[i],
                flat_input_as_bytes[i + line_length],
                flat_input_as_bytes[i + 2 * line_length],
                flat_input_as_bytes[i + 3 * line_length],
            ] == XMAS_BYTES
            {
                count += 1;
            }
        }
    }

    println!("{count}")
}

fn solve_part2(input: &String) {
    let mut count = 0;
    let mut flat_input = String::new();
    let mut line_length = 0;
    for line in input.lines() {
        line_length = line.len();
        flat_input.push_str(line);
    }
    let full_length = flat_input.len();
    let flat_input_as_bytes: &[u8] = flat_input.as_bytes();
    let mut x_mas_vec: Vec<char> = vec!['O'; full_length];
    for i in 0..flat_input_as_bytes.len() {
        if flat_input_as_bytes[i] != MAS_BYTES[0] {
            continue;
        }
        // check for boundaries
        let r: bool = check_right(i, line_length, MAS_LENGTH);
        let l: bool = check_left(i, line_length, MAS_LENGTH);
        let u: bool = check_up(i, line_length, MAS_LENGTH);
        let d: bool = check_down(i, line_length, full_length, MAS_LENGTH);

        //println!(
        //    "index: {} - boundaries right: {}, left: {}, up: {}, down: {} ",
        //    i, r, l, u, d
        //);
        if r {
            if u {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i - line_length + 1],
                    flat_input_as_bytes[i - 2 * line_length + 2],
                ] == MAS_BYTES
                {
                    if x_mas_vec[i - line_length + 1] == 'X' {
                        count += 1;
                    } else {
                        x_mas_vec[i - line_length + 1] = 'X'
                    }
                }
            }
            if d {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i + line_length + 1],
                    flat_input_as_bytes[i + 2 * line_length + 2],
                ] == MAS_BYTES
                {
                    if x_mas_vec[i + line_length + 1] == 'X' {
                        count += 1;
                    } else {
                        x_mas_vec[i + line_length + 1] = 'X'
                    }
                }
            }
        }
        if l {
            if u {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i - line_length - 1],
                    flat_input_as_bytes[i - 2 * line_length - 2],
                ] == MAS_BYTES
                {
                    if x_mas_vec[i - line_length - 1] == 'X' {
                        count += 1;
                    } else {
                        x_mas_vec[i - line_length - 1] = 'X'
                    }
                }
            }
            if d {
                if [
                    flat_input_as_bytes[i],
                    flat_input_as_bytes[i + line_length - 1],
                    flat_input_as_bytes[i + 2 * line_length - 2],
                ] == MAS_BYTES
                {
                    if x_mas_vec[i + line_length - 1] == 'X' {
                        count += 1;
                    } else {
                        x_mas_vec[i + line_length - 1] = 'X'
                    }
                }
            }
        }
    }
    println!("{count}")
}

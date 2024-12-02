use std::fs::File;
use std::io::prelude::*;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let mut file = File::open("day02/input.txt").map_err(|err| {
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

enum Direction {
    Up,
    Down,
    Neutral,
}

fn is_safe(mut l: Vec<&str>) -> bool {
    let mut prev = 0;
    let mut direction: Direction = Direction::Neutral;
    while !l.is_empty() {
        let curr = l.pop().unwrap().parse::<u32>().unwrap();
        if prev == curr {
            return false;
        }
        if prev == 0 {
            prev = curr;
            continue;
        }
        match direction {
            Direction::Neutral => {
                if prev > curr {
                    direction = Direction::Down
                } else {
                    direction = Direction::Up
                }
            }
            Direction::Down => {
                if curr > prev {
                    return false;
                }
            }
            Direction::Up => {
                if curr < prev {
                    return false;
                }
            }
        }
        if curr.abs_diff(prev) > 3 {
            return false;
        }
        if l.is_empty() {
            return true;
        }
        prev = curr;
    }
    return false;
}

fn solve_part1(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let l: Vec<&str> = line.split(" ").collect();
        if is_safe(l) {
            count += 1
        }
    }
    println!("{count}")
}

fn solve_part2(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let l: Vec<&str> = line.split(" ").collect();
        if is_safe(l.clone()) {
            count += 1
        } else {
            for i in 0..l.len() {
                let mut l_without_i = l.clone();
                l_without_i.remove(i);
                if is_safe(l_without_i) {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("{count}")
}

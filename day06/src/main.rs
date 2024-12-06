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
    //solve_part2(&contents);
    Ok(())
}

fn solve_part1(_input: &String) {}

fn _solve_part2(_input: &String) {}

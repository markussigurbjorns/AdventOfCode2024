use std::fs::File;
use std::result;
use std::io::prelude::*;
use std::collections::HashMap;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
   let mut file = File::open("day01/input.txt").map_err(|err|{
      eprintln!("ERROR: could not open file {err}");
   })?;
   let mut contents = String::new();
   file.read_to_string(&mut contents).map_err(|err|{
      eprintln!("ERROR: could not map contents of a file to a string {err}");
   })?;
   solve_part1(contents.clone());
   solve_part2(contents.clone());
   Ok(())
}

fn solve_part1(input: String) {
   let mut column1 = Vec::new();
   let mut column2 = Vec::new();
   for line in input.lines() {
      let l: Vec<&str> = line.split("   ").collect();
      column1.push(l[0].parse::<u32>().unwrap());
      column2.push(l[1].parse::<u32>().unwrap());
   }
   column1.sort();
   column2.sort();
   let mut sum = 0;
   while !column1.is_empty() {
      sum += column1.pop().unwrap().abs_diff(column2.pop().unwrap());
   }
   println!("{sum}")
}

fn solve_part2 (input: String) {
   let mut column = Vec::new();
   let mut frequency_map = HashMap::new();

   for line in input.lines() {
      let l: Vec<&str> = line.split("   ").collect();
      column.push(l[0]);
      *frequency_map.entry(l[1]).or_insert(0) += 1;
   }
   let mut sum = 0;
   while !column.is_empty() {
      let key = &column.pop().unwrap();
      match frequency_map.get(key) {
         Some(&value) => sum += value * key.parse::<u32>().unwrap(),
         None => ()
      }
   }
   println!("{sum}")
}

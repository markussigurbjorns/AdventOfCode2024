use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::{result, usize};

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let mut file = File::open("day06/input.txt").map_err(|err| {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    position: (i32, i32),
    direction: (i32, i32),
}

impl Guard {
    fn new(pos: (i32, i32), dir: (i32, i32)) -> Self {
        return Self {
            position: pos,
            direction: dir,
        };
    }

    fn new_empty() -> Self {
        return Self {
            position: (0, 0),
            direction: (0, -1),
        };
    }

    fn set_init_pos(&mut self, x: i32, y: i32) {
        self.position = (x, y)
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_direction(&self) -> (i32, i32) {
        self.direction
    }

    fn _print(&self) {
        println!(
            "my position is at {:?},{:?}",
            self.position.0, self.position.1
        );
        println!("my direction is {:?}", self.direction)
    }

    fn move_guard(&mut self) {
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
    }

    fn peek_pos(&self) -> (i32, i32) {
        (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        )
    }

    fn change_direction(&mut self) {
        match self.direction {
            (0, -1) => self.direction = (1, 0),
            (1, 0) => self.direction = (0, 1),
            (0, 1) => self.direction = (-1, 0),
            (-1, 0) => self.direction = (0, -1),
            _ => println!("something is wrong"),
        }
    }
}

fn dist_squared(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    dx * dx + dy * dy
}

fn is_rectangle(points: &VecDeque<(i32, i32)>, point: &(i32, i32)) -> bool {
    if points.len() != 3 {
        return false;
    }
    let a = points[0];
    let b = points[1];
    let c = points[2];
    let d = *point;
    if (a.0 + c.0) != (b.0 + d.0) || (a.1 + c.1) != (b.1 + d.1) {
        return false;
    }

    let diag_ac = dist_squared(a, c);
    let diag_bd = dist_squared(b, d);

    diag_ac == diag_bd
}

fn solve_part1(input: &String) {
    let mut map = Vec::new();
    let mut guard = Guard::new_empty();
    let mut col_len: i32 = 0;
    let mut row_len: i32 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut tmp = Vec::new();
        row_len = i as i32;
        for (j, l) in line.chars().enumerate() {
            tmp.push(l);
            if l == '^' {
                guard.set_init_pos(j as i32, i as i32);
            }
            col_len = j as i32;
        }
        map.push(tmp);
    }
    let mut traversed_map = Vec::new();
    let init_pos = guard.get_position();
    traversed_map.push(init_pos);

    loop {
        let (x, y) = guard.peek_pos();
        //guard.print();
        if x < 0 || x > row_len || y < 0 || y > col_len {
            break;
        }

        if map[y as usize][x as usize] == '#' {
            guard.change_direction();
        }

        guard.move_guard();
        let curr_pos = guard.get_position();
        if !traversed_map.contains(&curr_pos) {
            traversed_map.push(curr_pos.clone());
        }
    }
    println!("{:?}", traversed_map.len());
}

fn solve_part2(input: &String) {
    let mut count = 0;
    let mut map = Vec::new();
    let mut guard = Guard::new_empty();
    let mut col_len: i32 = 0;
    let mut row_len: i32 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut tmp = Vec::new();
        row_len = i as i32;
        for (j, l) in line.chars().enumerate() {
            tmp.push(l);
            if l == '^' {
                guard.set_init_pos(j as i32, i as i32);
            }
            col_len = j as i32;
        }
        map.push(tmp);
    }
    let mut traversed_map = Vec::new();
    let mut obsticle_map = Vec::new();

    loop {
        let (x, y) = guard.peek_pos();
        //guard.print();
        if x < 0 || x > row_len || y < 0 || y > col_len {
            println!("Reached the edge at: ({}, {})", x, y);
            break;
        }

        let curr_pos = guard.get_position();
        let curr_dir = guard.get_direction();
        traversed_map.push(Guard::new(curr_pos, curr_dir));
        let mut clone_guard = guard.clone();
        clone_guard.change_direction();
        let mut clone_traversed_map = traversed_map.clone();
        loop {
            let (xx, yy) = clone_guard.peek_pos();
            if xx < 0 || xx > row_len || yy < 0 || yy > col_len {
                //println!("Clone reached the edge at: ({}, {})", x, y);
                break;
            }

            if clone_traversed_map.contains(&clone_guard) {
                if !obsticle_map.contains(&clone_guard) {
                    println!("{:?}", clone_guard);
                    obsticle_map.push(clone_guard);
                    count += 1;
                    break;
                }
                println!("found duplicate");
                clone_guard.move_guard();
                //continue;
            }
            
            let clone_curr_pos = clone_guard.get_position();
            let clone_curr_dir = clone_guard.get_direction();
            clone_traversed_map.push(Guard::new(clone_curr_pos, clone_curr_dir));

            if map[yy as usize][xx as usize] == '#' {
                clone_guard.change_direction();
            }
            clone_guard.move_guard();
        }
        if map[y as usize][x as usize] == '#' {
            guard.change_direction();
            continue;
        }
        guard.move_guard();
    }
    println!("{count}")
}

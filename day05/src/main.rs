use std::{cell::RefCell, rc::Rc};

use std::fs::File;
use std::io::prelude::*;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let mut file = File::open("day05/input.txt").map_err(|err| {
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

#[derive(Debug, Clone)]
struct PageNode {
    page: u32,
    children: Vec<PageNodeRef>,
}

impl PageNode {
    fn new(page: u32) -> Self {
        Self {
            page: page,
            children: Vec::new(),
        }
    }

    fn insert_child(&mut self, page_node: PageNodeRef) {
        self.children.push(page_node)
    }

    fn has_child(&self, page: u32) -> bool {
        for child in self.children.clone() {
            if child.as_ref().borrow().page == page {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct PageGraph {
    nodes: Vec<PageNodeRef>,
}

impl PageGraph {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn get_node(&self, &page: &u32) -> Option<PageNodeRef> {
        for node in self.nodes.clone() {
            if node.as_ref().borrow().page == page {
                return Some(node);
            }
        }
        return None;
    }

    fn insert_node(&mut self, page_node: PageNodeRef) {
        self.nodes.push(page_node)
    }

    fn has_node(&self, page: u32) -> bool {
        for node in self.nodes.clone() {
            if node.as_ref().borrow().page == page {
                return true;
            }
        }
        false
    }
}

type PageNodeRef = Rc<RefCell<PageNode>>;

fn solve_part1(input: &String) {
    let mut graph: PageGraph = PageGraph::new();
    let mut data = vec![75, 47, 61, 53, 29];
    let mut process = false;
    for line in input.lines() {
        if process {
            let nl = line.eq("");
            if nl {
                process = true;
                continue;
            }
            //println!("{line}");
            let l: Vec<&str> = line.split("|").collect();

            let left = l[0].parse::<u32>().unwrap();
            let right = l[1].parse::<u32>().unwrap();

            if graph.has_node(left) {
                let node = graph.get_node(&left).unwrap();
                node.clone()
                    .as_ref()
                    .borrow_mut()
                    .insert_child(PageNodeRef::new(PageNode::new(right).into()));
            } else {
                let mut left_node = PageNode::new(left);
                let right_node = PageNodeRef::new(PageNode::new(right).into());
                left_node.insert_child(right_node);
                graph.insert_node(PageNodeRef::new(left_node.into()));
            }
        } else {
            // FIX ME FINISH THIS
            for i in 0..data.len() {
                let curr = data[i];
                println!("{curr}");
                if i + 1 == data.len() {
                    println!("is not correct");
                    break;
                }
                if graph.has_node(curr) {
                    let node = graph.get_node(&curr).unwrap();

                    let next = data[i + 1];
                    if node.clone().as_ref().borrow().has_child(next) {
                        println!("is correct")
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn _solve_part2(_input: &String) {}

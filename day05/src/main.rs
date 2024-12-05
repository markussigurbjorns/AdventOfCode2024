use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::result;
use std::{cell::RefCell, rc::Rc};

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
    solve_part2_v2(&contents);
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
        for child in &self.children {
            if child.borrow().page == page {
                return true;
            }
        }
        false
    }

    fn max_depth(&self) -> u32 {
        let pp = self.page;
        println!("im on page {pp}");
        if self.children.is_empty() {
            1
        } else {
            1 + self
                .children
                .iter()
                .map(|child_ref| {
                    let child = child_ref.borrow();
                    child.max_depth()
                })
                .max()
                .unwrap()
        }
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
        for node in &self.nodes {
            if node.borrow().page == page {
                return Some(node.clone());
            }
        }
        return None;
    }

    fn insert_node(&mut self, page_node: PageNodeRef) {
        self.nodes.push(page_node)
    }

    fn has_node(&self, page: u32) -> bool {
        for node in &self.nodes {
            if node.borrow().page == page {
                return true;
            }
        }
        false
    }
}

type PageNodeRef = Rc<RefCell<PageNode>>;

fn permutations(items: &[u32]) -> Vec<Vec<u32>> {
    let mut results = Vec::new();
    let mut arr = items.to_vec();
    permute_recursive(0, &mut arr, &mut results);
    results
}

fn permute_recursive(start: usize, arr: &mut [u32], results: &mut Vec<Vec<u32>>) {
    if start == arr.len() {
        results.push(arr.to_vec());
    } else {
        for i in start..arr.len() {
            arr.swap(start, i);
            permute_recursive(start + 1, arr, results);
            arr.swap(start, i);
        }
    }
}

fn solve_part1(input: &String) {
    let mut sum = 0;
    let mut graph: PageGraph = PageGraph::new();
    let mut process = false;
    for (index, line) in input.lines().enumerate() {
        if !process {
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
                node.as_ref()
                    .borrow_mut()
                    .insert_child(PageNodeRef::new(PageNode::new(right).into()));
            } else {
                let mut left_node = PageNode::new(left);
                let right_node = PageNodeRef::new(PageNode::new(right).into());
                left_node.insert_child(right_node);
                graph.insert_node(PageNodeRef::new(left_node.into()));
            }
        } else {
            let l: Vec<&str> = line.split(",").collect();
            for i in 0..l.len() {
                if i == l.len() - 1 {
                    sum += l[l.len() / 2].parse::<u32>().unwrap();
                    continue;
                }
                let curr = l[i].parse::<u32>().unwrap();
                if graph.has_node(curr) {
                    let node = graph.get_node(&curr).unwrap();
                    if i + 1 < l.len() {
                        let next = l[i + 1].parse::<u32>().unwrap();
                        if !node.as_ref().borrow().has_child(next) {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
    println!("{sum}")
}

fn solve_part2(input: &String) {
    let mut sum = 0;
    let banned_indexes = vec![
        1177, 1178, 1179, 1180, 1182, 1185, 1186, 1189, 1191, 1192, 1193, 1194, 1195, 1198, 1201,
        1204, 1205, 1206, 1208, 1209, 1210, 1212, 1213, 1215, 1216, 1217, 1220, 1222, 1223, 1226,
        1229, 1230, 1231, 1233, 1234, 1236, 1237, 1240, 1241, 1242, 1243, 1244, 1245, 1247, 1248,
        1250, 1252, 1256, 1258, 1260, 1261, 1263, 1264, 1268, 1269, 1270, 1271, 1272, 1273, 1274,
        1275, 1277, 1278, 1280, 1282, 1283, 1284, 1289, 1294, 1295, 1296, 1297, 1298, 1304, 1305,
        1306, 1307, 1308, 1310, 1312, 1313, 1314, 1315, 1316, 1317, 1318, 1321, 1324, 1325, 1326,
        1327, 1328, 1332, 1333, 1335, 1336, 1337, 1338, 1343, 1344, 1345, 1346, 1348, 1349, 1354,
        1356, 1357, 1358, 1360, 1361, 1362, 1364, 1365,
    ];
    let mut graph: PageGraph = PageGraph::new();
    let mut process = false;
    for (index, line) in input.lines().enumerate() {
        if banned_indexes.contains(&index) {
            continue;
        }
        if !process {
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
                node.as_ref()
                    .borrow_mut()
                    .insert_child(PageNodeRef::new(PageNode::new(right).into()));
            } else {
                let mut left_node = PageNode::new(left);
                let right_node = PageNodeRef::new(PageNode::new(right).into());
                left_node.insert_child(right_node);
                graph.insert_node(PageNodeRef::new(left_node.into()));
            }
        } else {
            println!("{index}");
            let ll: Vec<&str> = line.split(",").collect();

            let mut l = Vec::new();

            for j in ll {
                l.push(j.parse::<u32>().unwrap());
            }
            //let perms = permutations(&l);
            for p in l.iter().permutations(l.len()) {
                for i in 0..p.len() {
                    if i == p.len() - 1 {
                        sum += p[p.len() / 2];
                        continue;
                    }
                    let curr = p[i];
                    if graph.has_node(*curr) {
                        let node = graph.get_node(&curr).unwrap();
                        if i + 1 < p.len() {
                            let next = p[i + 1];
                            if !node.as_ref().borrow().has_child(*next) {
                                break;
                            }
                        }
                    } else {
                        //println!("do I get here?");
                        break;
                    }
                }
            }
        }
    }
    println!("{sum}")
}

fn build_graph(nodes: &[u32], edges: &[(u32, u32)]) -> (HashMap<u32, Vec<u32>>, HashMap<u32, u32>) {
    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut indegree: HashMap<u32, u32> = HashMap::new();

    // Initialize adjacency and indegree
    for &node in nodes {
        adj.insert(node, Vec::new());
        indegree.insert(node, 0);
    }
    for &(u, v) in edges {
        adj.entry(u).or_insert(Vec::new()).push(v);
        indegree.entry(u).or_insert(0); // Ensure source node is in the indegree map
        indegree.entry(v).or_insert(0); // Ensure target node is in the indegree map
        *indegree.get_mut(&v).unwrap() += 1; // Increment in-degree for target node
    }

    (adj, indegree)
}

fn topological_sort(edges: &[(u32, u32)], nodes: &[u32]) -> Option<Vec<u32>> {
    // Step 1: Build the graph (including implicit nodes from edges)
    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut indegree: HashMap<u32, u32> = HashMap::new();

    // Ensure all nodes (explicit and implicit) are accounted for
    for &(u, v) in edges {
        adj.entry(u).or_insert(Vec::new()).push(v);
        indegree.entry(u).or_insert(0); // Add source node if missing
        indegree.entry(v).or_insert(0); // Add target node if missing
        *indegree.get_mut(&v).unwrap() += 1; // Increment in-degree for target
    }

    for &node in nodes {
        adj.entry(node).or_insert(Vec::new());
        indegree.entry(node).or_insert(0);
    }

    // Step 2: Initialize a queue for nodes with zero in-degree
    let mut queue = VecDeque::new();
    for (&node, &deg) in &indegree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    // Step 3: Perform topological sort
    let mut result = Vec::new();
    while let Some(u) = queue.pop_front() {
        result.push(u);

        // For each neighbor, reduce its in-degree
        if let Some(neighbors) = adj.get(&u) {
            for &neighbor in neighbors {
                let in_deg = indegree.get_mut(&neighbor).unwrap();
                *in_deg -= 1;
                if *in_deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Step 4: Check for cycles
    if result.len() == indegree.len() {
        Some(result) // All nodes processed, valid topological order
    } else {
        None // Cycle detected
    }
}

//Kahn’s algorithm for Topological Sorting
fn solve_part2_v2(input: &String) {
    let mut edges = Vec::new();
    let mut process = false;
    let banned_indexes = vec![
        1177, 1178, 1179, 1180, 1182, 1185, 1186, 1189, 1191, 1192, 1193, 1194, 1195, 1198, 1201,
        1204, 1205, 1206, 1208, 1209, 1210, 1212, 1213, 1215, 1216, 1217, 1220, 1222, 1223, 1226,
        1229, 1230, 1231, 1233, 1234, 1236, 1237, 1240, 1241, 1242, 1243, 1244, 1245, 1247, 1248,
        1250, 1252, 1256, 1258, 1260, 1261, 1263, 1264, 1268, 1269, 1270, 1271, 1272, 1273, 1274,
        1275, 1277, 1278, 1280, 1282, 1283, 1284, 1289, 1294, 1295, 1296, 1297, 1298, 1304, 1305,
        1306, 1307, 1308, 1310, 1312, 1313, 1314, 1315, 1316, 1317, 1318, 1321, 1324, 1325, 1326,
        1327, 1328, 1332, 1333, 1335, 1336, 1337, 1338, 1343, 1344, 1345, 1346, 1348, 1349, 1354,
        1356, 1357, 1358, 1360, 1361, 1362, 1364, 1365,
    ];
    for (index, line) in input.lines().enumerate() {
        if banned_indexes.contains(&index) {
            continue;
        }
        if !process {
            let nl = line.eq("");
            if nl {
                process = true;
                continue;
            }
            let l: Vec<&str> = line.split("|").collect();
            edges.push((l[0].parse::<u32>().unwrap(), l[1].parse::<u32>().unwrap()))
        } else {
            let l: Vec<&str> = line.split(",").collect();

            let mut nodes = Vec::new();

            for node in l {
                nodes.push(node.parse::<u32>().unwrap());
            }

            //let (adj, mut indegree) = build_graph(&nodes, &edges);

            if let Some(sorted) = topological_sort(&edges, &nodes) {
                println!("Sorted order: {:?}", sorted);
            } else {
                println!("No valid topological order (cycle detected).");
            }
        }
    }
}

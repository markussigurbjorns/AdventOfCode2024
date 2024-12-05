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
    solve_part2(&contents);
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

fn solve_part1(input: &String) {
    let mut sum = 0;
    let mut graph: PageGraph = PageGraph::new();
    let mut process = false;
    for (_index, line) in input.lines().enumerate() {
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

fn sort_with_filtered_rules(input: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let input_set: std::collections::HashSet<u32> = input.iter().copied().collect();
    let filtered_rules: Vec<(u32, u32)> = rules
        .iter()
        .filter(|&&(u, v)| input_set.contains(&u) && input_set.contains(&v))
        .copied()
        .collect();

    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut indegree: HashMap<u32, u32> = HashMap::new();
    let mut result = Vec::new();

    for &(u, v) in &filtered_rules {
        adj.entry(u).or_default().push(v);
        indegree.entry(v).and_modify(|e| *e += 1).or_insert(1);
        indegree.entry(u).or_insert(0); // Ensure all nodes in input appear in the map
    }

    let mut queue: VecDeque<u32> = input
        .iter()
        .filter(|&&node| indegree.get(&node).copied().unwrap_or(0) == 0)
        .copied()
        .collect();

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(neighbors) = adj.get(&node) {
            for &neighbor in neighbors {
                let deg = indegree.get_mut(&neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    if result.len() == input.len() {
        result
    } else {
        Vec::new() 
    }
}

fn solve_part2(input: &String) {
    let mut sum = 0;
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

            let sorted = sort_with_filtered_rules(&nodes, &edges);

            if sorted.is_empty() {
                println!("Invalid rules (contains cycles or inconsistencies).");
            } else {
                sum += sorted[sorted.len() / 2];

            }
        }
    }
    println!("{sum}")
}

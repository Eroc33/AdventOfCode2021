use advent_of_utils::Error;
use day12::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::BufRead,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Node {
    fn is_small(&self) -> bool {
        matches!(self, Node::Small(_))
    }
    fn is_big(&self) -> bool {
        matches!(self, Node::Big(_))
    }
    fn is_start(&self) -> bool {
        matches!(self, Node::Start)
    }
    fn is_end(&self) -> bool {
        matches!(self, Node::End)
    }
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Self::Start,
            "end" => Self::End,
            small if small.chars().all(char::is_lowercase) => Self::Small(small.into()),
            big if big.chars().all(char::is_uppercase) => Self::Big(big.into()),
            other => return Err(format!("Unexpected node: {:?}", other).into()),
        })
    }
}

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut nodes: HashSet<Node> = HashSet::new();
    let mut edges: HashMap<Node, HashSet<Node>> = HashMap::new();
    for line in input.lines() {
        let line = line?;
        let line = line.trim();
        let (a, b) = advent_of_utils::split_parse::<Node, Node>(line, "-")?;
        nodes.insert(a.clone());
        nodes.insert(b.clone());
        edges.entry(a.clone()).or_default().insert(b.clone());
        edges.entry(b).or_default().insert(a);
    }
    dbg!(&nodes);
    dbg!(&edges);
    let mut completed_paths = HashSet::new();
    let mut open_paths = HashSet::new();
    open_paths.insert(vec![Node::Start]);

    while !open_paths.is_empty() {
        for path in std::mem::take(&mut open_paths) {
            if let Some(next_nodes) = edges.get(path.last().unwrap()) {
                'adding_nodes: for next_node in next_nodes {
                    if next_node.is_start() || (next_node.is_small() && path.contains(next_node)) {
                        continue 'adding_nodes;
                    }
                    let mut new_path = path.clone();
                    new_path.push(next_node.clone());
                    if next_node.is_end() {
                        completed_paths.insert(new_path);
                    } else {
                        open_paths.insert(new_path);
                    }
                }
            }
        }
    }

    Ok(completed_paths.len() as usize)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day12_part1_example1() {
    advent_of_utils::check_example(
        solution,
        "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end",
        10,
    )
}

#[cfg(test)]
#[test]
fn day12_part1_example2() {
    advent_of_utils::check_example(
        solution,
        "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc",
        19,
    )
}

#[cfg(test)]
#[test]
fn day12_part1_example3() {
    advent_of_utils::check_example(
        solution,
        "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW",
        226,
    )
}

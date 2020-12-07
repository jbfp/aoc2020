use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};

static FILE: &str = include_str!("../inputs/7.txt");

lazy_static! {
    static ref LINES: Vec<(&'static str, Vec<&'static str>)> = FILE
        .lines()
        .map(|line| {
            let (key, values) = line.split_once("contain").unwrap();
            let key = key.strip_suffix("s ").unwrap_or(key).trim();
            let values = values
                .split(',')
                .map(|value| &value.trim_end_matches('.').trim_end_matches('s')[1..])
                .collect::<Vec<_>>();

            (key, values)
        })
        .collect();
}

pub(crate) fn part1() {
    println!("day 7 part 1");

    let mut graph: DiGraph<&str, u32> = Graph::new();
    let mut nodes = HashMap::new();

    // Add all nodes
    for (key, _) in LINES.iter() {
        let idx = graph.add_node(key);
        nodes.insert(*key, idx);
    }

    // Add all edges
    for (key, values) in LINES.iter() {
        let key = nodes[*key];

        for value in values {
            if *value == "no other bag" {
                continue;
            }

            let (weight, value) = value.split_once(' ').unwrap();
            let value = nodes[value];
            graph.update_edge(key, value, weight.parse().unwrap());
        }
    }

    let my_bag = nodes["shiny gold bag"];

    let num_paths = graph
        .node_indices()
        .filter(|from| petgraph::algo::has_path_connecting(&graph, *from, my_bag, None))
        .count();

    println!("num paths to shiny gold bag: {}", num_paths - 1);
}

pub(crate) fn part2() {
    println!("day 7 part 2");

    let mut nodes = HashMap::new();

    for (key, values) in LINES.iter() {
        let mut set = HashSet::new();

        for value in values {
            if *value == "no other bag" {
                continue;
            }

            let (weight, value) = value.split_once(' ').unwrap();
            set.insert((value, weight.parse::<u32>().unwrap()));
        }

        nodes.insert(*key, set);
    }

    println!(
        "number of bags inside shiny my bag: {}",
        calculate_num_bags(&nodes, "shiny gold bag") - 1
    );
}

fn calculate_num_bags(nodes: &HashMap<&str, HashSet<(&str, u32)>>, key: &str) -> u32 {
    let set = &nodes[key];
    let mut result = 1;

    for (key, num) in set {
        result += num * calculate_num_bags(nodes, key);
    }

    result
}

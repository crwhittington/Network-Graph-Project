use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn graph_builder(filename: &str) -> Vec<Vec<usize>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut graph = vec![vec![]; 0];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let u = iter.next().unwrap().parse::<usize>().unwrap();
        let v = iter.next().unwrap().parse::<usize>().unwrap();

        let max_node_id = u.max(v);
        for _i in graph.len()..=max_node_id {
            graph.push(vec![]);
        }

        graph[u].push(v);
    }

    graph
}
use std::collections::{HashSet, VecDeque};

pub fn kosaraju(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut order = VecDeque::new();
    let mut components = Vec::new();
    let mut stack = Vec::new();

    for i in 0..n {
        if !visited[i] {
            stack.push(i);
            while let Some(v) = stack.pop() {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                    for &w in &graph[v] {
                        if !visited[w] {
                            stack.push(w);
                        }
                    }
                } else {
                    order.push_front(v);
                }
            }
        }
    }

    visited = vec![false; n];
    while let Some(v) = order.pop_front() {
        if !visited[v] {
            let mut component = HashSet::new();
            stack.push(v);
            while let Some(u) = stack.pop() {
                if !visited[u] {
                    visited[u] = true;
                    component.insert(u);
                    for &w in &graph[u] {
                        if !visited[w] {
                            stack.push(w);
                        }
                    }
                }
            }
            components.push(component.into_iter().collect());
        }
    }

    components
}


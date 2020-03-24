use std::io::{self, Write};
use std::vec::Vec;

struct Edge {
    source: i64,
    destination: i64,
    cost: i64
}

fn make_sets(number_of_nodes: i64) -> Vec<i64> {
    let mut parent: Vec<i64> = Vec::with_capacity(number_of_nodes as usize);
    for i in 0..number_of_nodes {
        parent.push(i);
    }
    return parent;
}

fn find(parent: &mut Vec<i64>, x: i64) -> i64 {
    let idx: usize = x as usize;
    if parent[idx] != x {
        parent[idx] = find(parent, parent[idx]);
    }
    return parent[idx];
}

fn merge(parent: &mut Vec<i64>, x: i64, y: i64) {
    let idx_x: usize = find(parent, x) as usize;
    let parent_y: i64 = find(parent, y);
    parent[idx_x] = parent_y;
}

fn is_same_set(parent: &mut Vec<i64>, x: i64, y: i64) -> bool {
    return find(parent, x) == find(parent, y);
}

fn main() {

    io::stdout().flush().unwrap();
}

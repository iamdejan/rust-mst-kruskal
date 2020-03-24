use std::io::{self, Write};
use std::vec::Vec;
use std::string::String;
use std::fmt;

struct Edge {
    source: i64,
    destination: i64,
    cost: i64
}

impl Edge {
    fn new(source: i64, destination: i64, cost: i64) -> Self {
        return Self {
            source: source,
            destination: destination,
            cost: cost
        };
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "{} {} {}", self.source, self.destination, self.cost);
    }
}

fn get_line() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");
    return buffer;
}

fn make_sets(number_of_vertices: i64) -> Vec<i64> {
    let mut parent: Vec<i64> = Vec::with_capacity(number_of_vertices as usize);
    for i in 0..number_of_vertices {
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

fn kruskal(mut edges: Vec<Edge>, mut parent: Vec<i64>, number_of_vertices: i64) -> (i64, Vec<Edge>) {
    edges.sort_by(|a, b| a.cost.cmp(&b.cost));
    let mut total_cost: i64 = 0;
    let mut final_edges: Vec<Edge> = Vec::new();
    let mut merge_count: i64 = 0;
    for edge in edges.iter() {
        if merge_count >= number_of_vertices - 1 {
            break;
        }

        let source: i64 = edge.source;
        let destination: i64 = edge.destination;
        if is_same_set(&mut parent, source, destination) != true {
            merge(&mut parent, source, destination);
            merge_count += 1;
            let cost: i64 = edge.cost;
            total_cost += cost;
            let final_edge: Edge = Edge::new(source, destination, cost);
            final_edges.push(final_edge);
        }
    }
    return (total_cost, final_edges);
}

fn main() {
    let number_of_vertices: i64 = get_line().trim().parse().unwrap();
    let number_of_edges: i64 = get_line().trim().parse().unwrap();

    let parent: Vec<i64> = make_sets(number_of_vertices);
    let mut edges: Vec<Edge> = Vec::new();

    println!("Edges:");
    for _ in 0..number_of_edges {
        let line: String = get_line().trim().to_string();
        let numbers: Vec<i64> = line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
        let source = numbers[0];
        let destination = numbers[1];
        let cost = numbers[2];
        println!("{} {} {}", source, destination, cost);
        let edge: Edge = Edge::new(source, destination, cost);
        edges.push(edge);
    }
    println!("");

    let (total_cost, final_edges) = kruskal(edges, parent, number_of_vertices);
    println!("Total cost: {}", total_cost);
    println!("Edges used in MST:");
    for edge in final_edges.iter() {
        println!("{}", edge);
    }

    io::stdout().flush().unwrap();
}

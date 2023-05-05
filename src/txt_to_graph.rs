use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph{
    pub edges: Vec<(usize,usize)>,
    pub nodes: Vec<usize>,
    pub num_nodes: usize,
}

pub fn read_as_graph(file_path : &str) -> Graph{
    let file = File::open(file_path).expect("Failed to open");
        let reader = BufReader::new(file);
        let mut edges: Vec<(usize,usize)> = Vec::new();
        let mut nodes: Vec<usize> = Vec::new();
    
        for line in reader.lines() {
            let line = line.expect("Failed to read");
            let parts: Vec<&str> = line.split(" ").collect();
            let vertex1 = parts[0].parse().expect("Failed to parse v1");
            let vertex2 = parts[1].parse().expect("Failed to parse v2");

            edges.push((vertex1, vertex2));
            nodes.push(vertex1);
            nodes.push(vertex2);
        }
        //sort the nodes and delete the repeated ones
        edges.sort();
        nodes.sort();
        nodes.dedup();
        let num_nodes = nodes.len();
        Graph { nodes, edges, num_nodes }
}

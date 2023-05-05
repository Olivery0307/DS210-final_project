use std::collections::HashMap;

// show the distributions of neighbors
pub fn distribution_of_neighbors(adjacency_list: HashMap<usize, Vec<usize>>) -> Vec<(usize, u32)> {

    let mut neighbor_distribution = Vec::new();

    for neighbors in adjacency_list.values() {
        let degree = neighbors.len();
        let index = neighbor_distribution.iter().position(|&(d, _)| d == degree);

        match index {
            Some(i) => {
                let (_, count) = &mut neighbor_distribution[i];
                *count += 1;
            }
            None => neighbor_distribution.push((degree, 1)),
        }
    }
    neighbor_distribution.sort_by_key(|&(degree, _)| degree);
    neighbor_distribution
}


//compute degree distribution
pub fn calculate_degree_distribution(neignbor_distribution: Vec<(usize, u32)>, num_nodes:usize) -> Vec<(usize, f64)> {

    let mut degree_distribution = Vec::new();

    for &(degree, count) in &neignbor_distribution {
        let frequency = count as f64 / num_nodes as f64;
        degree_distribution.push((degree, frequency));
    }
    degree_distribution
}
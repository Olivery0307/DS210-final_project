mod txt_to_graph;

mod degree_distribution;

mod six_degrees_separation;

//Create an adjacent list to store neighbors of nodes(use for both degree distribution and six degrees of separation)
use std::collections::HashMap;
pub fn find_neighbors(edges: Vec<(usize,usize)>) -> HashMap<usize, Vec<usize>>  {
    
        let mut adjacency_list = HashMap::new();
        
        for (v,w) in edges.iter() {
            adjacency_list.entry(*v).or_insert_with(Vec::new).push(*w);
            adjacency_list.entry(*w).or_insert_with(Vec::new).push(*v);
        }
        adjacency_list
    }



fn main() {
        let graph = txt_to_graph::read_as_graph("facebook.txt");
        let edges = graph.edges;
        let num_nodes = graph.num_nodes;
        let adj_list = find_neighbors(edges);

        //degree distribution
        let neighbors = degree_distribution::distribution_of_neighbors(adj_list.clone());
        let degree_dis = degree_distribution::calculate_degree_distribution(neighbors,num_nodes);
        println!("Degree Distribution:");
        for (node, frequency) in degree_dis {
            println!("Degree {}: {}", node, frequency);
        }

        println!("=====================");

        //six_degrees of separation
        println!("Six Degrees of Separation:");

        for i in 0..num_nodes{

            let distance = six_degrees_separation::bfs_for_distance(num_nodes,adj_list.clone(),i);

            println!("Numbers of nodes by distances from node {}", i+1);
            let six_degrees = six_degrees_separation::count_by_distance(distance);
            println!("{:?}",six_degrees)
    }
        
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_find_neighbors(){
        let mut test_result: HashMap<usize, Vec<usize>> = HashMap::new();
        test_result.extend(vec![(1,vec![2,3]),(2,vec![1]),(3,vec![1])]);
        assert_eq!(find_neignbors(vec![(1,2),(1,3)]),test_result);
    }

    #[test]
    fn test_distribution_of_neighbors(){
        let mut test2_hash: HashMap<usize, Vec<usize>> = HashMap::new();
        test2_hash.extend(vec![(1,vec![2,3]),(2,vec![1,4]),(3,vec![1]),(4,vec![2,5]),(5,vec![4])]);
        let mut test2_result: HashMap<usize, u32> = HashMap::new();
        test2_result.extend(vec![(1,2),(2,3)]);
        assert_eq!(degree_distribution::distribution_of_neighbors(test2_hash),test2_result);
    }

    #[test]
    fn test_calculate_degree_deistribution(){
        let test_num_nodes = 5;
        let mut test3_hash: HashMap<usize, u32> = HashMap::new();
        test3_hash.extend(vec![(1,2),(2,3)]);
        let mut test3_result: HashMap<usize,f64> = HashMap::new();
        test3_result.extend(vec![(1,0.4),(2,0.6)]);
        assert_eq!(degree_distribution::calculate_degree_distribution(test3_hash,test_num_nodes),test3_result);

    }

    #[test]
    fn test_distance_between_nodes(){
        let test_num_nodes:usize = 6;
        let mut test_outedges:HashMap<usize, Vec<usize>> = HashMap::new();
        test_outedges.extend(vec![(1,vec![2,3]),(2,vec![1,4]),(3,vec![1]),(4,vec![2,5]),(5,vec![4])]);
        let test_start:usize = 1;
        let mut test4_result: HashMap<usize,usize> = HashMap::new();
        test4_result.extend(vec![(1,0),(2,1),(3,1),(4,2),(5,3)]);
        assert_eq!(six_degrees_separation::distance_between_nodes(test_num_nodes,test_outedges,test_start),test4_result);
    }

    #[test]
    fn test_count_by_distance(){
        let mut test_distances: HashMap<usize, usize> = HashMap::new();
        test_distances.extend(vec![(1,0),(2,1),(3,1),(4,2),(5,3)]);
        let mut test5_result: HashMap<usize,usize> = HashMap::new();
        test5_result.extend(vec![(0,1),(1,2),(2,1),(3,1)]);
        assert_eq!(six_degrees_separation::count_by_distance(test_distances),test5_result);
    }
}
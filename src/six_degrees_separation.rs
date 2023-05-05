use std::collections::{HashMap,VecDeque};

//this function returns the distance between each nodes and starting point
pub fn bfs_for_distance(num_nodes:usize, outedges:HashMap<usize, Vec<usize>> ,start : usize) -> HashMap<usize, usize>{
    
    let mut distance: Vec<Option<usize>> = vec![None;num_nodes];
    distance[start] = Some(0); 
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut nodes_distance = HashMap::new();

    queue.push_back(start);

    while let Some(v) = queue.pop_front(){
       if let Some(neighbors) = outedges.get(&v) {
        for u in neighbors.iter(){
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u); 
                }
            }
        }
    }

    for (i,dis) in distance.into_iter().enumerate(){
        if let Some(dist) = dis {
            nodes_distance.insert(i, dist);
        }
    }
    nodes_distance
}

//count numbers of each nodes by distance
pub fn count_by_distance(distances: HashMap<usize, usize>) -> HashMap<usize, usize> {

    let mut count_map = HashMap::new();

    for &dis in distances.values(){

        *count_map.entry(dis).or_insert(0) += 1;

    } 
    count_map
}

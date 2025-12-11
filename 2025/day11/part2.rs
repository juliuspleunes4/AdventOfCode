// >> DAY 11 - PART 2 <<

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input");
    
    // Parse the graph
    let graph = parse_graph(&input);
    
    println!("Graph has {} nodes", graph.len());
    
    // Count all paths from "svr" to "out" that visit both "dac" and "fft"
    let count = count_paths_with_required(&graph, "svr", "out", vec!["dac", "fft"]);
    
    println!("Number of paths from 'svr' to 'out' visiting both 'dac' and 'fft': {}", count);
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let node = parts[0].trim().to_string();
        let destinations: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        graph.insert(node, destinations);
    }
    
    graph
}

fn count_paths_with_required(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    required: Vec<&str>,
) -> usize {
    // Build reverse graph to find all nodes that can reach 'out'
    let mut reverse_graph: HashMap<String, Vec<String>> = HashMap::new();
    for (node, neighbors) in graph {
        for neighbor in neighbors {
            reverse_graph.entry(neighbor.clone()).or_insert_with(Vec::new).push(node.clone());
        }
    }
    
    // Find nodes reachable from start
    let reachable_from_start = bfs_reachable(graph, start);
    
    // Find nodes that can reach end
    let can_reach_end = bfs_reachable(&reverse_graph, end);
    
    // Only explore nodes in the intersection
    let valid_nodes: HashSet<String> = reachable_from_start
        .intersection(&can_reach_end)
        .cloned()
        .collect();
    
    println!("Valid nodes to explore: {}", valid_nodes.len());
    
    // Now do DFS only on valid nodes
    let mut memo: HashMap<(String, u8), usize> = HashMap::new();
    
    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        current: &str,
        end: &str,
        required: &[&str],
        required_mask: u8,
        valid_nodes: &HashSet<String>,
        memo: &mut HashMap<(String, u8), usize>,
        depth: usize,
    ) -> usize {
        if depth > 100 {
            return 0; // Prevent stack overflow
        }
        
        // Update required mask
        let mut new_mask = required_mask;
        for (i, &req) in required.iter().enumerate() {
            if current == req {
                new_mask |= 1 << i;
            }
        }
        
        // If we reached the end
        if current == end {
            let all_required = (1 << required.len()) - 1;
            return if new_mask == all_required { 1 } else { 0 };
        }
        
        // Check memo
        let key = (current.to_string(), new_mask);
        if let Some(&result) = memo.get(&key) {
            return result;
        }
        
        let mut total = 0;
        
        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                if valid_nodes.contains(neighbor) {
                    total += dfs(graph, neighbor, end, required, new_mask, valid_nodes, memo, depth + 1);
                }
            }
        }
        
        memo.insert(key, total);
        total
    }
    
    dfs(graph, start, end, &required, 0, &valid_nodes, &mut memo, 0)
}

fn bfs_reachable(graph: &HashMap<String, Vec<String>>, start: &str) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start.to_string());
    visited.insert(start.to_string());
    
    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    
    visited
}


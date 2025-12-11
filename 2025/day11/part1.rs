// >> DAY 11 - PART 1 <<

use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input");
    
    // Parse the graph
    let graph = parse_graph(&input);
    
    // Count all paths from "you" to "out"
    let count = count_paths(&graph, "you", "out");
    
    println!("Number of paths from 'you' to 'out': {}", count);
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

fn count_paths(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    fn dfs(
        graph: &HashMap<String, Vec<String>>,
        current: &str,
        end: &str,
        visited: &mut Vec<String>,
    ) -> usize {
        // If we reached the end, we found one path
        if current == end {
            return 1;
        }
        
        // Mark current as visited
        visited.push(current.to_string());
        
        let mut total_paths = 0;
        
        // Explore all neighbors
        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                // Only visit if not already in the current path (to avoid cycles)
                if !visited.contains(neighbor) {
                    total_paths += dfs(graph, neighbor, end, visited);
                }
            }
        }
        
        // Backtrack: remove current from visited
        visited.pop();
        
        total_paths
    }
    
    let mut visited = Vec::new();
    dfs(graph, start, end, &mut visited)
}

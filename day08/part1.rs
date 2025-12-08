// >> DAY 08 - PART 1 <<

use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Parse junction box positions
    let mut boxes: Vec<(i32, i32, i32)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<i32> = line.split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        boxes.push((parts[0], parts[1], parts[2]));
    }
    
    let n = boxes.len();
    
    // Calculate all pairwise distances
    let mut edges: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            let dx = (boxes[i].0 - boxes[j].0) as i64;
            let dy = (boxes[i].1 - boxes[j].1) as i64;
            let dz = (boxes[i].2 - boxes[j].2) as i64;
            let dist_sq = (dx*dx + dy*dy + dz*dz) as u64;
            edges.push((dist_sq, i, j));
        }
    }
    
    // Sort edges by distance
    edges.sort_by_key(|e| e.0);
    
    // Union-Find data structure
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];
    
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    
    fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let rx = find(parent, x);
        let ry = find(parent, y);
        
        if rx == ry {
            return false;
        }
        
        if size[rx] < size[ry] {
            parent[rx] = ry;
            size[ry] += size[rx];
        } else {
            parent[ry] = rx;
            size[rx] += size[ry];
        }
        true
    }
    
    // Connect the 1000 closest pairs (count all attempts, not just successful unions)
    let mut attempts = 0;
    let mut successful = 0;
    for (_, i, j) in edges {
        if union(&mut parent, &mut size, i, j) {
            successful += 1;
        }
        attempts += 1;
        if attempts == 1000 {
            break;
        }
    }
    
    println!("Made {} connection attempts, {} successful", attempts, successful);
    
    // Find all circuit sizes
    let mut circuit_sizes: Vec<usize> = Vec::new();
    for i in 0..n {
        if find(&mut parent, i) == i {
            circuit_sizes.push(size[i]);
        }
    }
    
    // Sort to find the three largest
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    
    println!("Number of circuits: {}", circuit_sizes.len());
    println!("Top 3 circuit sizes: {:?}", &circuit_sizes[..circuit_sizes.len().min(3)]);
    
    if circuit_sizes.len() >= 3 {
        let result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
        println!("Result: {}", result);
    } else {
        println!("Not enough circuits!");
    }
}

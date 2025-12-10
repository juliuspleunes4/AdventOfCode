// >> DAY 08 - PART 2 <<

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
    let mut num_circuits = n;
    
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    
    fn union(parent: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let rx = find(parent, x);
        let ry = find(parent, y);
        
        if rx == ry {
            return false;
        }
        
        parent[ry] = rx;
        true
    }
    
    // Connect pairs until all in one circuit
    for (_, i, j) in edges {
        if union(&mut parent, i, j) {
            num_circuits -= 1;
            
            if num_circuits == 1 {
                // This was the last connection needed
                let result = (boxes[i].0 as i64) * (boxes[j].0 as i64);
                println!("Last connection: {} <-> {}", i, j);
                println!("Positions: {:?} <-> {:?}", boxes[i], boxes[j]);
                println!("Result: {} * {} = {}", boxes[i].0, boxes[j].0, result);
                break;
            }
        }
    }
}

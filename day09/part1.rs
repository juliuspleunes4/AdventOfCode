// >> DAY 09 - PART 1 <<

use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Parse red tile positions - use i64 to avoid overflow
    let mut tiles: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<i64> = line.split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        tiles.push((parts[0], parts[1]));
    }
    
    let n = tiles.len();
    println!("Number of tiles: {}", n);
    
    let mut max_area = 0i64;
    
    // Check all pairs of tiles as opposite corners
    for i in 0..n {
        for j in i+1..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            
            // Rectangle is inclusive of both corners, so add 1 to each dimension
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;
            
            if area > max_area {
                max_area = area;
            }
        }
    }
    
    println!("Largest rectangle area: {}", max_area);
}

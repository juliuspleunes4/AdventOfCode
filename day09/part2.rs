// >> DAY 09 - PART 2 <<

use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Parse red tile positions - they form a loop in order
    let mut red_tiles: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<i64> = line.split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        red_tiles.push((parts[0], parts[1]));
    }
    
    let n = red_tiles.len();
    let red_set: HashSet<(i64, i64)> = red_tiles.iter().cloned().collect();
    
    println!("Number of red tiles: {}", n);
    
    let mut max_area = 0i64;
    
    // Check all pairs of red tiles as opposite corners
    for i in 0..n {
        if i % 50 == 0 {
            println!("Checking tile {} of {}...", i, n);
        }
        
        for j in i+1..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];
            
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            
            // Check if the entire rectangle is inside/on the polygon
            // We need to verify all 4 corners AND edges don't escape the polygon
            
            let corners = [
                (min_x, min_y),
                (min_x, max_y),
                (max_x, min_y),
                (max_x, max_y),
            ];
            
            // All corners must be on polygon boundary or inside
            let mut valid = true;
            for &(x, y) in &corners {
                if !is_on_polygon_or_inside(x, y, &red_tiles, &red_set) {
                    valid = false;
                    break;
                }
            }
            
            // Additionally check points along the edges AND interior to ensure entire rectangle is valid
            if valid {
                let width = max_x - min_x;
                let height = max_y - min_y;
                
                // Sample more densely for smaller rectangles, less for larger ones
                let sample_step = (width.max(height) / 100).max(1);
                
                // Check all edge and interior points
                for y in (min_y..=max_y).step_by(sample_step as usize) {
                    for x in (min_x..=max_x).step_by(sample_step as usize) {
                        if !is_on_polygon_or_inside(x, y, &red_tiles, &red_set) {
                            valid = false;
                            break;
                        }
                    }
                    if !valid { break; }
                }
                
                // Also check the actual edges precisely
                if valid {
                    for x in min_x..=max_x {
                        if !is_on_polygon_or_inside(x, min_y, &red_tiles, &red_set) ||
                           !is_on_polygon_or_inside(x, max_y, &red_tiles, &red_set) {
                            valid = false;
                            break;
                        }
                    }
                }
                
                if valid {
                    for y in min_y..=max_y {
                        if !is_on_polygon_or_inside(min_x, y, &red_tiles, &red_set) ||
                           !is_on_polygon_or_inside(max_x, y, &red_tiles, &red_set) {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            
            if valid {
                let area = (max_x - min_x + 1) * (max_y - min_y + 1);
                if area > max_area {
                    max_area = area;
                    println!("New max: {} at ({},{}) to ({},{})", area, x1, y1, x2, y2);
                }
            }
        }
    }
    
    println!("\nLargest valid rectangle area: {}", max_area);
}

// Check if a point is on the polygon boundary (red or green edge tiles) or strictly inside
fn is_on_polygon_or_inside(px: i64, py: i64, polygon: &[(i64, i64)], red_set: &HashSet<(i64, i64)>) -> bool {
    // Check if it's a red tile
    if red_set.contains(&(px, py)) {
        return true;
    }
    
    // Check if it's on an edge between consecutive red tiles (green tile)
    let n = polygon.len();
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];
        
        if x1 == x2 && px == x1 {
            // Vertical edge
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            if py >= min_y && py <= max_y {
                return true;
            }
        } else if y1 == y2 && py == y1 {
            // Horizontal edge
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            if px >= min_x && px <= max_x {
                return true;
            }
        }
    }
    
    // Check if it's strictly inside the polygon
    point_in_polygon(px, py, polygon)
}

// Check if a point is strictly inside the polygon using ray casting
fn point_in_polygon(px: i64, py: i64, polygon: &[(i64, i64)]) -> bool {
    let n = polygon.len();
    let mut inside = false;
    
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

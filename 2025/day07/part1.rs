// >> DAY 07 - PART 1 <<

use std::fs;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let height = grid.len();
    let width = grid[0].len();
    
    // Find starting position S
    let mut start_col = 0;
    for col in 0..width {
        if grid[0][col] == 'S' {
            start_col = col;
            break;
        }
    }
    
    // Track beams: (row, col, direction)
    // Direction: 0=down, 1=left, 2=right
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut split_count = 0;
    
    // Start with initial beam going down from S
    queue.push_back((0, start_col, 0));
    
    while let Some((row, col, dir)) = queue.pop_front() {
        // Skip if we've already processed this beam state
        if visited.contains(&(row, col, dir)) {
            continue;
        }
        visited.insert((row, col, dir));
        
        // Check current position
        let current_char = grid[row][col];
        
        if current_char == '^' {
            // ANY beam hit a splitter - count this split
            split_count += 1;
            
            // Create two new beams from immediate left and right of splitter, moving DOWNWARD
            if col > 0 {
                queue.push_back((row, col - 1, 0)); // beam at left position, moving DOWN
            }
            if col < width - 1 {
                queue.push_back((row, col + 1, 0)); // beam at right position, moving DOWN
            }
        } else {
            // Empty space or S - continue moving downward
            let next_row = row + 1;
            
            // Check if next position is in bounds
            if next_row < height {
                queue.push_back((next_row, col, 0)); // always move down
            }
        }
    }
    
    println!("Total splits: {}", split_count);
}

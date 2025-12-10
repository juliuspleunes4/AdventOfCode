// >> DAY 07 - PART 2 <<

use std::fs;
use std::collections::HashMap;

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
    
    // Count paths with memoization
    fn count_paths(grid: &Vec<Vec<char>>, row: usize, col: usize, 
                   memo: &mut HashMap<(usize, usize), u64>, height: usize, width: usize) -> u64 {
        // Check memo
        if let Some(&result) = memo.get(&(row, col)) {
            return result;
        }
        
        let current_char = grid[row][col];
        let result;
        
        if current_char == '^' {
            // Hit a splitter - sum paths through both branches
            let mut total = 0u64;
            if col > 0 {
                total += count_paths(grid, row, col - 1, memo, height, width);
            }
            if col < width - 1 {
                total += count_paths(grid, row, col + 1, memo, height, width);
            }
            result = total;
        } else {
            // Empty space or S - continue downward
            let next_row = row + 1;
            
            if next_row < height {
                result = count_paths(grid, next_row, col, memo, height, width);
            } else {
                // Exited the manifold - this is one complete path
                result = 1;
            }
        }
        
        memo.insert((row, col), result);
        result
    }
    
    let mut memo = HashMap::new();
    let timeline_count = count_paths(&grid, 0, start_col, &mut memo, height, width);
    
    println!("Total timelines: {}", timeline_count);
}

// >> DAY 04 - PART 1 <<

use std::fs;

fn count_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;
    
    // Check all 8 adjacent positions
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];
    
    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;
        
        // Check bounds
        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    
    count
}

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Parse the grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    
    let mut accessible_count = 0;
    
    // Check each position
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                let neighbor_count = count_neighbors(&grid, row, col);
                if neighbor_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }
    
    println!("Accessible rolls: {}", accessible_count);
}

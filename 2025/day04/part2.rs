// >> DAY 04 - PART 2 <<

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

fn find_accessible(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                let neighbor_count = count_neighbors(&grid, row, col);
                if neighbor_count < 4 {
                    accessible.push((row, col));
                }
            }
        }
    }
    
    accessible
}

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Parse the grid
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    
    let mut total_removed = 0;
    
    // Keep removing accessible rolls until no more can be removed
    loop {
        let accessible = find_accessible(&grid);
        
        if accessible.is_empty() {
            break;
        }
        
        // Remove all accessible rolls
        for (row, col) in accessible.iter() {
            grid[*row][*col] = '.';
        }
        
        total_removed += accessible.len();
    }
    
    println!("Total rolls removed: {}", total_removed);
}

// >> DAY 06 - PART 2 <<

use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.len() < 2 {
        return;
    }
    
    // Convert to grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    let max_width = lines.iter().map(|line| line.len()).max().unwrap();
    
    for line in lines {
        let mut row: Vec<char> = line.chars().collect();
        while row.len() < max_width {
            row.push(' ');
        }
        grid.push(row);
    }
    
    let height = grid.len();
    let width = grid[0].len();
    let op_row = height - 1;
    
    // Find separator columns (columns that are ALL spaces)
    let mut separators = vec![true; width];
    for col in 0..width {
        for row in 0..height {
            if grid[row][col] != ' ' {
                separators[col] = false;
                break;
            }
        }
    }
    
    // Find problem blocks (ranges between separators)
    let mut problems: Vec<(usize, usize, char)> = Vec::new();
    
    let mut col = 0;
    while col < width {
        if !separators[col] {
            let start = col;
            let mut end = col;
            
            while end < width && !separators[end] {
                end += 1;
            }
            end -= 1;
            
            // Find the operation
            let mut op = ' ';
            for c in start..=end {
                if grid[op_row][c] != ' ' {
                    op = grid[op_row][c];
                    break;
                }
            }
            
            if op != ' ' {
                problems.push((start, end, op));
            }
            
            col = end + 1;
        } else {
            col += 1;
        }
    }
    
    let mut grand_total: u128 = 0;
    
    for (start, end, op) in problems {
        let mut numbers: Vec<u128> = Vec::new();
        
        // For each column (reading right-to-left), read digits top-to-bottom to form a number
        for col in (start..=end).rev() {
            let mut num_str = String::new();
            
            // Read rows from top to bottom (most significant digit first)
            for row in 0..op_row {
                if grid[row][col] != ' ' {
                    num_str.push(grid[row][col]);
                }
            }
            
            if !num_str.is_empty() {
                numbers.push(num_str.parse().unwrap());
            }
        }
        
        // Calculate result
        if !numbers.is_empty() {
            let result = if op == '+' {
                numbers.iter().sum::<u128>()
            } else if op == '*' {
                numbers.iter().product::<u128>()
            } else {
                0
            };
            
            grand_total += result;
        }
    }
    
    println!("Grand total: {}", grand_total);
}

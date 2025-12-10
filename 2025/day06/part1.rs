// >> DAY 06 - PART 1 <<

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
    let mut problems: Vec<(usize, usize, char)> = Vec::new(); // (start, end, operation)
    
    let mut col = 0;
    while col < width {
        if !separators[col] {
            // Start of a problem block
            let start = col;
            let mut end = col;
            
            // Find end of this problem block
            while end < width && !separators[end] {
                end += 1;
            }
            end -= 1;
            
            // Find the operation in this block
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
        
        // Collect all numbers in this problem block
        for row in 0..op_row {
            let mut c = start;
            while c <= end {
                if grid[row][c].is_digit(10) {
                    let mut num_str = String::new();
                    while c <= end && grid[row][c].is_digit(10) {
                        num_str.push(grid[row][c]);
                        c += 1;
                    }
                    numbers.push(num_str.parse().unwrap());
                } else {
                    c += 1;
                }
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

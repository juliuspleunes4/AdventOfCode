// >> DAY 03 - PART 1 <<

use std::fs;

fn max_joltage(bank: &str) -> u32 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();
    
    let mut max_value = 0;
    
    // Try all pairs of positions (i, j) where i < j
    // The number formed is digits[i] followed by digits[j]
    for i in 0..n {
        for j in (i+1)..n {
            let first = digits[i].to_digit(10).unwrap();
            let second = digits[j].to_digit(10).unwrap();
            let value = first * 10 + second;
            
            if value > max_value {
                max_value = value;
            }
        }
    }
    
    max_value
}

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    let mut total_joltage = 0;
    
    // Process each bank (line)
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let joltage = max_joltage(line);
        total_joltage += joltage;
    }
    
    println!("Total output joltage: {}", total_joltage);
}

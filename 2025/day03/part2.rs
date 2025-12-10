// >> DAY 03 - PART 2 <<

use std::fs;

fn max_joltage_part2(bank: &str, keep_count: usize) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();
    
    if n == keep_count {
        return bank.parse().unwrap();
    }
    
    // Build the largest number by keeping the lexicographically largest subsequence
    let mut result = Vec::new();
    let mut start = 0;
    
    for _ in 0..keep_count {
        // We need to pick 'remaining' more digits
        let remaining = keep_count - result.len();
        // We can look ahead up to (n - remaining) positions
        let end = n - remaining + 1;
        
        // Find the largest digit in the valid range
        let mut max_digit = '0';
        let mut max_pos = start;
        
        for i in start..end {
            if digits[i] > max_digit {
                max_digit = digits[i];
                max_pos = i;
            }
        }
        
        result.push(max_digit);
        start = max_pos + 1;
    }
    
    let result_str: String = result.iter().collect();
    result_str.parse().unwrap()
}

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    let mut total_joltage: u64 = 0;
    
    // Process each bank (line)
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let joltage = max_joltage_part2(line, 12);
        total_joltage += joltage;
    }
    
    println!("Total output joltage: {}", total_joltage);
}

// >> DAY 02 - PART 1 <<

use std::fs;

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    
    // Must be even length to be splittable
    if len % 2 != 0 {
        return false;
    }
    
    let half = len / 2;
    let first_half = &id_str[0..half];
    let second_half = &id_str[half..];
    
    // Check if both halves are identical
    first_half == second_half
}

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    let mut total_sum: u64 = 0;
    
    // Parse the comma-separated ranges
    for range in input.trim().split(',') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let start: u64 = parts[0].parse().expect("Failed to parse start");
        let end: u64 = parts[1].parse().expect("Failed to parse end");
        
        // Check each ID in the range
        for id in start..=end {
            if is_invalid_id(id) {
                total_sum += id;
            }
        }
    }
    
    println!("Sum of all invalid IDs: {}", total_sum);
}

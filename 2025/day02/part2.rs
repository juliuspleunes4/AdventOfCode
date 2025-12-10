// >> DAY 02 - PART 2 <<

use std::fs;

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    
    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len/2 {
        // Check if the entire string can be made by repeating this pattern
        if len % pattern_len == 0 {
            let pattern = &id_str[0..pattern_len];
            let repetitions = len / pattern_len;
            
            // Build what the string would be if pattern repeated
            let repeated = pattern.repeat(repetitions);
            
            // If it matches and repeats at least twice, it's invalid
            if repeated == id_str && repetitions >= 2 {
                return true;
            }
        }
    }
    
    false
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

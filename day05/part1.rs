// >> DAY 05 - PART 1 <<

use std::fs;

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Split by double newlines to separate sections
    let sections: Vec<&str> = input.split("\r\n\r\n").collect();
    
    let sections = if sections.len() == 2 {
        sections
    } else {
        input.split("\n\n").collect()
    };
    
    if sections.len() != 2 {
        panic!("Expected two sections separated by blank line, got {}", sections.len());
    }
    
    // Parse fresh ingredient ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in sections[0].lines() {
        let parts: Vec<&str> = line.trim().split('-').collect();
        if parts.len() == 2 {
            let start: u64 = parts[0].parse().expect("Failed to parse start");
            let end: u64 = parts[1].parse().expect("Failed to parse end");
            ranges.push((start, end));
        }
    }
    
    // Parse available ingredient IDs
    let mut available_ids: Vec<u64> = Vec::new();
    for line in sections[1].lines() {
        let line = line.trim();
        if !line.is_empty() {
            available_ids.push(line.parse().expect("Failed to parse ID"));
        }
    }
    
    // Count how many available IDs are fresh
    let mut fresh_count = 0;
    for id in available_ids {
        // Check if ID falls in any range
        let mut is_fresh = false;
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                is_fresh = true;
                break;
            }
        }
        
        if is_fresh {
            fresh_count += 1;
        }
    }
    
    println!("Fresh ingredient IDs: {}", fresh_count);
}

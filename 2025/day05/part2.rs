// >> DAY 05 - PART 2 <<

use std::fs;

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Split by double newline 
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
    
    // Sort ranges by start position
    ranges.sort_by_key(|r| r.0);
    
    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    
    for (start, end) in ranges {
        if merged.is_empty() {
            merged.push((start, end));
        } else {
            let last_idx = merged.len() - 1;
            let (last_start, last_end) = merged[last_idx];
            
            // Check if current range overlaps or is adjacent to last range
            if start <= last_end + 1 {
                // Merge by extending the end if needed
                merged[last_idx] = (last_start, last_end.max(end));
            } else {
                // No overlap, add as new range
                merged.push((start, end));
            }
        }
    }
    
    // Count total IDs in merged ranges
    let mut total_count: u64 = 0;
    for (start, end) in merged {
        total_count += end - start + 1;
    }
    
    println!("Total fresh ingredient IDs: {}", total_count);
}

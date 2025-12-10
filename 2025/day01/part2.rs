// >> DAY 01 - PART 2 <<

use std::fs;

fn main() {
    // Read the puzzle input
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    // Starting position
    let mut position: i32 = 50;
    let mut zero_count = 0;
    
    // Process each rotation instruction
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse direction (first character) and distance (rest of the string)
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");
        
        // Count how many times we pass through or land on 0 during this rotation
        // We check each click from 1 to distance
        match direction {
            'L' => {
                // Count crossings by checking each position
                for i in 1..=distance {
                    if (position - i).rem_euclid(100) == 0 {
                        zero_count += 1;
                    }
                }
                position = (position - distance).rem_euclid(100);
            },
            'R' => {
                // Count crossings by checking each position
                for i in 1..=distance {
                    if (position + i).rem_euclid(100) == 0 {
                        zero_count += 1;
                    }
                }
                position = (position + distance).rem_euclid(100);
            },
            _ => panic!("Unknown direction: {}", direction),
        }
    }
    
    println!("The password is: {}", zero_count);
}

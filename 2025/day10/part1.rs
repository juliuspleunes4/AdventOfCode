// >> DAY 10 - PART 1 <<

use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input file");
    
    let mut total_presses = 0;
    
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        // Parse the line
        let (target, buttons) = parse_line(line);
        
        // Find minimum button presses using brute force
        let min_presses = solve_lights(&target, &buttons);
        total_presses += min_presses;
    }
    
    println!("Total minimum button presses: {}", total_presses);
}

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    // Remove the 'f' prefix if present
    let line = line.trim_start_matches('f');
    
    // Extract target pattern from [...]
    let start = line.find('[').unwrap();
    let end = line.find(']').unwrap();
    let target_str = &line[start+1..end];
    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();
    
    // Extract button patterns from (...)
    let mut buttons = Vec::new();
    let remaining = &line[end+1..];
    
    for part in remaining.split('{').next().unwrap().split('(').skip(1) {
        if let Some(close) = part.find(')') {
            let nums: Vec<usize> = part[..close]
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if !nums.is_empty() {
                buttons.push(nums);
            }
        }
    }
    
    (target, buttons)
}

fn solve_lights(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let num_lights = target.len();
    let num_buttons = buttons.len();
    
    // For small number of buttons, try all combinations
    if num_buttons <= 20 {
        let mut min_presses = num_buttons + 1;
        
        for mask in 0..(1 << num_buttons) {
            let mut state = vec![false; num_lights];
            let mut presses = 0;
            
            for btn_idx in 0..num_buttons {
                if (mask & (1 << btn_idx)) != 0 {
                    presses += 1;
                    for &light_idx in &buttons[btn_idx] {
                        if light_idx < num_lights {
                            state[light_idx] = !state[light_idx];
                        }
                    }
                }
            }
            
            if state == target {
                min_presses = min_presses.min(presses);
            }
        }
        
        return min_presses;
    }
    
    num_buttons
}

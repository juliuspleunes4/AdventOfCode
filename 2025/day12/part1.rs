// >> DAY 12 - PART 1 <<

use std::collections::HashSet;
use std::fs;

type Grid = Vec<Vec<bool>>;

#[derive(Clone, Debug)]
struct Shape {
    cells: Vec<(i32, i32)>, // positions of '#' relative to origin
}

impl Shape {
    fn from_grid(grid: &[Vec<char>]) -> Self {
        let mut cells = Vec::new();
        for (row, line) in grid.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                if ch == '#' {
                    cells.push((row as i32, col as i32));
                }
            }
        }
        // Normalize to have min coordinates at (0,0)
        let min_row = cells.iter().map(|&(r, _)| r).min().unwrap_or(0);
        let min_col = cells.iter().map(|&(_, c)| c).min().unwrap_or(0);
        cells = cells.iter().map(|&(r, c)| (r - min_row, c - min_col)).collect();
        Shape { cells }
    }
    
    fn rotate(&self) -> Shape {
        // Rotate 90 degrees clockwise: (r, c) -> (c, -r)
        let mut cells: Vec<(i32, i32)> = self.cells.iter().map(|&(r, c)| (c, -r)).collect();
        let min_row = cells.iter().map(|&(r, _)| r).min().unwrap_or(0);
        let min_col = cells.iter().map(|&(_, c)| c).min().unwrap_or(0);
        cells = cells.iter().map(|&(r, c)| (r - min_row, c - min_col)).collect();
        Shape { cells }
    }
    
    fn flip_horizontal(&self) -> Shape {
        // Flip horizontally: (r, c) -> (r, -c)
        let mut cells: Vec<(i32, i32)> = self.cells.iter().map(|&(r, c)| (r, -c)).collect();
        let min_row = cells.iter().map(|&(r, _)| r).min().unwrap_or(0);
        let min_col = cells.iter().map(|&(_, c)| c).min().unwrap_or(0);
        cells = cells.iter().map(|&(r, c)| (r - min_row, c - min_col)).collect();
        Shape { cells }
    }
    
    fn all_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::new();
        let mut seen = HashSet::new();
        
        let mut current = self.clone();
        for _ in 0..4 {
            let key = format!("{:?}", current.cells);
            if seen.insert(key) {
                orientations.push(current.clone());
            }
            current = current.rotate();
        }
        
        current = self.flip_horizontal();
        for _ in 0..4 {
            let key = format!("{:?}", current.cells);
            if seen.insert(key) {
                orientations.push(current.clone());
            }
            current = current.rotate();
        }
        
        orientations
    }
}

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt")
        .expect("Failed to read puzzle input");
    
    let (shapes, regions) = parse_input(&input);
    
    println!("Parsed {} shapes and {} regions", shapes.len(), regions.len());
    
    let mut count = 0;
    for (i, (width, height, presents)) in regions.iter().enumerate() {
        let total: usize = presents.iter().sum();
        print!("Region {} ({}x{}, {} presents)... ", i + 1, width, height, total);
        if can_fit_all(*width, *height, &presents, &shapes) {
            count += 1;
            println!("[YES]");
        } else {
            println!("[NO]");
        }
    }
    
    println!("\nRegions that can fit all presents: {}", count);
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut shapes = Vec::new();
    
    // Parse shapes
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }
        
        if line.ends_with(':') {
            // Shape definition
            i += 1;
            let mut grid = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].contains(':') && !lines[i].contains('x') {
                grid.push(lines[i].chars().collect());
                i += 1;
            }
            if !grid.is_empty() {
                shapes.push(Shape::from_grid(&grid));
            }
        } else {
            break;
        }
    }
    
    // Parse regions
    let mut regions = Vec::new();
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }
        
        if line.contains('x') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dims: Vec<&str> = parts[0].split('x').collect();
                let width: usize = dims[0].parse().unwrap();
                let height: usize = dims[1].parse().unwrap();
                
                let counts: Vec<usize> = parts[1]
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                
                regions.push((width, height, counts));
            }
        }
        i += 1;
    }
    
    (shapes, regions)
}

fn can_fit_all(width: usize, height: usize, presents: &[usize], shapes: &[Shape]) -> bool {
    let mut grid = vec![vec![false; width]; height];
    let mut present_list = Vec::new();
    
    for (shape_idx, &count) in presents.iter().enumerate() {
        for _ in 0..count {
            present_list.push(shape_idx);
        }
    }
    
    // Sort by size (largest first) for better pruning
    present_list.sort_by_key(|&idx| std::cmp::Reverse(shapes[idx].cells.len()));
    
    // Quick check: total area
    let total_area: usize = present_list.iter().map(|&idx| shapes[idx].cells.len()).sum();
    if total_area > width * height {
        return false;
    }
    
    backtrack(&mut grid, &present_list, shapes, 0)
}

fn backtrack(grid: &mut Vec<Vec<bool>>, presents: &[usize], shapes: &[Shape], idx: usize) -> bool {
    if idx >= presents.len() {
        return true; // All presents placed
    }
    
    let shape_idx = presents[idx];
    let orientations = shapes[shape_idx].all_orientations();
    
    // Try placing this present at every position and orientation
    for orientation in &orientations {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if can_place(grid, orientation, row as i32, col as i32) {
                    place(grid, orientation, row as i32, col as i32, true);
                    if backtrack(grid, presents, shapes, idx + 1) {
                        return true;
                    }
                    place(grid, orientation, row as i32, col as i32, false);
                }
            }
        }
    }
    
    false
}

fn can_place(grid: &[Vec<bool>], shape: &Shape, start_row: i32, start_col: i32) -> bool {
    for &(dr, dc) in &shape.cells {
        let r = start_row + dr;
        let c = start_col + dc;
        
        if r < 0 || r >= grid.len() as i32 || c < 0 || c >= grid[0].len() as i32 {
            return false;
        }
        
        if grid[r as usize][c as usize] {
            return false;
        }
    }
    true
}

fn place(grid: &mut Vec<Vec<bool>>, shape: &Shape, start_row: i32, start_col: i32, value: bool) {
    for &(dr, dc) in &shape.cells {
        let r = (start_row + dr) as usize;
        let c = (start_col + dc) as usize;
        grid[r][c] = value;
    }
}

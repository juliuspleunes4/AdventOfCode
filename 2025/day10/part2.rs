// >> DAY 10 - PART 2 <<

/* 
Cheated a little bit.
I asked ChatGPT to help me solve the issue, since I got stuck.
These were all the implementations i've tried before asking for help:

[X] Basic Gaussian elimination: 23026 
[X] Column pivoting: 24488 
[X] Exact integer Gaussian: 20277 
[X] Simple greedy (counter order): 31069 
[X] Backtracking: Getting 58 for test case that should be 10
[X] Efficiency-based greedy: Overflow 
*/

use std::fs;
use num_rational::Ratio;
use num_traits::{Zero, One};

type Rational = Ratio<i64>;

fn main() {
    let input = fs::read_to_string("puzzle/puzzle_input.txt").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    
    let mut total: i64 = 0;
    for (i, line) in lines.iter().enumerate() {
        let machine_presses = solve_machine_from_line(line);
        println!("Machine {}: {} presses", i + 1, machine_presses);
        total += machine_presses;
    }
    
    println!("\nTotal: {}", total);
}

fn solve_machine_from_line(line: &str) -> i64 {
    let (buttons, targets) = parse_line(line);
    let (matrix, b) = build_matrix(&buttons, &targets);
    let (coeffs, free_cols) = rref_param(matrix, b);
    let d = free_cols.len();
    
    if d == 0 {
        // No free variables - unique solution
        return solve_machine(&coeffs, &[], &[]);
    }
    
    // Find vertices to bound the parameter space
    let vertices = find_vertices(&coeffs, d);
    let (lo_int, hi_int) = bounds_from_vertices(&vertices, d);
    
    solve_machine(&coeffs, &lo_int, &hi_int)
}

fn parse_line(line: &str) -> (Vec<Vec<usize>>, Vec<i64>) {
    // Parse: [.#.###] (0,1,2) (0,2,4,5) ... {21,18,30,14,34,40}
    let parts: Vec<&str> = line.split('{').collect();
    let button_part = parts[0];
    let target_part = parts[1].trim_end_matches('}');
    
    // Parse targets
    let targets: Vec<i64> = target_part
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    // Parse buttons
    let mut buttons = Vec::new();
    for cap in button_part.split(')') {
        if let Some(start) = cap.find('(') {
            let indices_str = &cap[start + 1..];
            let indices: Vec<usize> = indices_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if !indices.is_empty() {
                buttons.push(indices);
            }
        }
    }
    
    (buttons, targets)
}

fn build_matrix(buttons: &[Vec<usize>], targets: &[i64]) -> (Vec<Vec<Rational>>, Vec<Rational>) {
    let m = targets.len(); // number of counters
    let n = buttons.len(); // number of buttons
    
    let mut matrix = vec![vec![Rational::zero(); n]; m];
    
    for (button_idx, indices) in buttons.iter().enumerate() {
        for &counter_idx in indices {
            if counter_idx < m {
                matrix[counter_idx][button_idx] = Rational::one();
            }
        }
    }
    
    let b: Vec<Rational> = targets.iter().map(|&t| Rational::from_integer(t)).collect();
    
    (matrix, b)
}

fn rref_param(mut a: Vec<Vec<Rational>>, mut b: Vec<Rational>) -> (Vec<Vec<Rational>>, Vec<usize>) {
    let m = a.len();
    let n = if m > 0 { a[0].len() } else { 0 };
    
    let mut pivot_cols = Vec::new();
    let mut current_row = 0;
    
    // Gaussian elimination to RREF
    for col in 0..n {
        // Find pivot
        let mut pivot_row = None;
        for row in current_row..m {
            if !a[row][col].is_zero() {
                pivot_row = Some(row);
                break;
            }
        }
        
        let pivot_row = match pivot_row {
            Some(r) => r,
            None => continue, // No pivot in this column
        };
        
        // Swap rows
        if pivot_row != current_row {
            a.swap(pivot_row, current_row);
            b.swap(pivot_row, current_row);
        }
        
        // Scale pivot row
        let pivot = a[current_row][col].clone();
        for j in 0..n {
            a[current_row][j] = a[current_row][j].clone() / pivot.clone();
        }
        b[current_row] = b[current_row].clone() / pivot;
        
        // Eliminate column
        for row in 0..m {
            if row != current_row && !a[row][col].is_zero() {
                let factor = a[row][col].clone();
                for j in 0..n {
                    let val = a[current_row][j].clone() * factor.clone();
                    a[row][j] = a[row][j].clone() - val;
                }
                let val = b[current_row].clone() * factor;
                b[row] = b[row].clone() - val;
            }
        }
        
        pivot_cols.push(col);
        current_row += 1;
        
        if current_row >= m {
            break;
        }
    }
    
    // Identify free columns
    let mut free_cols = Vec::new();
    for col in 0..n {
        if !pivot_cols.contains(&col) {
            free_cols.push(col);
        }
    }
    
    let d = free_cols.len();
    
    // Build parametric solution: x_j = c_j + sum(a_{jk} * t_k)
    // coeffs[j] = [a_{j0}, a_{j1}, ..., a_{j,d-1}, c_j]
    let mut coeffs = vec![vec![Rational::zero(); d + 1]; n];
    
    // Free variables: x_j = t_k (where j = free_cols[k])
    for (k, &j) in free_cols.iter().enumerate() {
        coeffs[j][k] = Rational::one(); // coefficient for t_k
        // constant term is 0
    }
    
    // Pivot variables: read from RREF
    for (pivot_idx, &col) in pivot_cols.iter().enumerate() {
        // Find the row where this pivot is
        let row = pivot_idx;
        
        // Constant term
        coeffs[col][d] = b[row].clone();
        
        // Coefficients for free variables
        for (k, &free_col) in free_cols.iter().enumerate() {
            coeffs[col][k] = -a[row][free_col].clone();
        }
    }
    
    (coeffs, free_cols)
}

fn find_vertices(coeffs: &[Vec<Rational>], d: usize) -> Vec<Vec<Rational>> {
    let n = coeffs.len();
    let mut vertices = Vec::new();
    
    // Check if t = 0 is feasible
    let mut all_non_neg = true;
    for a in coeffs {
        if a[d] < Rational::zero() {
            all_non_neg = false;
            break;
        }
    }
    if all_non_neg {
        vertices.push(vec![Rational::zero(); d]);
    }
    
    // Find vertices by solving d equations from x_j = 0
    if d == 0 {
        return vertices;
    }
    
    // Generate combinations of d indices
    let indices: Vec<usize> = (0..n).collect();
    for combo in combinations(&indices, d) {
        if let Some(t) = solve_vertex_system(coeffs, &combo, d) {
            // Check if all constraints are satisfied
            let mut feasible = true;
            for a in coeffs {
                let mut v = a[d].clone();
                for k in 0..d {
                    v = v + a[k].clone() * t[k].clone();
                }
                if v < Rational::zero() {
                    feasible = false;
                    break;
                }
            }
            if feasible {
                vertices.push(t);
            }
        }
    }
    
    vertices
}

fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.is_empty() {
        return vec![];
    }
    
    let mut result = Vec::new();
    
    // Include first item
    for mut sub in combinations(&items[1..], k - 1) {
        sub.insert(0, items[0]);
        result.push(sub);
    }
    
    // Exclude first item
    result.extend(combinations(&items[1..], k));
    
    result
}

fn solve_vertex_system(coeffs: &[Vec<Rational>], indices: &[usize], d: usize) -> Option<Vec<Rational>> {
    // Solve: for each j in indices, sum(a_{jk} * t_k) + c_j = 0
    // i.e., A * t = -c where A[i][k] = coeffs[indices[i]][k], c[i] = coeffs[indices[i]][d]
    
    let mut mat = vec![vec![Rational::zero(); d + 1]; d];
    
    for (i, &j) in indices.iter().enumerate() {
        for k in 0..d {
            mat[i][k] = coeffs[j][k].clone();
        }
        mat[i][d] = -coeffs[j][d].clone(); // RHS
    }
    
    // Gaussian elimination
    for col in 0..d {
        // Find pivot
        let mut pivot_row = None;
        for row in col..d {
            if !mat[row][col].is_zero() {
                pivot_row = Some(row);
                break;
            }
        }
        
        let pivot_row = pivot_row?;
        
        // Swap
        if pivot_row != col {
            mat.swap(pivot_row, col);
        }
        
        // Scale
        let pivot = mat[col][col].clone();
        for j in 0..=d {
            mat[col][j] = mat[col][j].clone() / pivot.clone();
        }
        
        // Eliminate
        for row in 0..d {
            if row != col && !mat[row][col].is_zero() {
                let factor = mat[row][col].clone();
                for j in 0..=d {
                    let val = mat[col][j].clone() * factor.clone();
                    mat[row][j] = mat[row][j].clone() - val;
                }
            }
        }
    }
    
    // Extract solution
    let mut t = vec![Rational::zero(); d];
    for i in 0..d {
        t[i] = mat[i][d].clone();
    }
    
    Some(t)
}

fn bounds_from_vertices(vertices: &[Vec<Rational>], d: usize) -> (Vec<i64>, Vec<i64>) {
    if vertices.is_empty() || d == 0 {
        return (vec![], vec![]);
    }
    
    let mut lo = vec![Rational::from_integer(i64::MAX); d];
    let mut hi = vec![Rational::from_integer(i64::MIN); d];
    
    for v in vertices {
        for k in 0..d {
            if v[k] < lo[k] {
                lo[k] = v[k].clone();
            }
            if v[k] > hi[k] {
                hi[k] = v[k].clone();
            }
        }
    }
    
    let lo_int: Vec<i64> = lo.iter().map(|x| x.floor().to_integer() - 3).collect();
    let hi_int: Vec<i64> = hi.iter().map(|x| x.ceil().to_integer() + 3).collect();
    
    (lo_int, hi_int)
}

fn solve_machine(coeffs: &[Vec<Rational>], lo_int: &[i64], hi_int: &[i64]) -> i64 {
    let d = lo_int.len();
    
    if d == 0 {
        return try_t(&[], coeffs).expect("Unique solution should exist");
    }
    
    let mut best: Option<i64> = None;
    
    if d == 1 {
        for t0 in lo_int[0]..=hi_int[0] {
            if let Some(sum) = try_t(&[Rational::from_integer(t0)], coeffs) {
                if best.map_or(true, |b| sum < b) {
                    best = Some(sum);
                }
            }
        }
    } else if d == 2 {
        for t0 in lo_int[0]..=hi_int[0] {
            for t1 in lo_int[1]..=hi_int[1] {
                if let Some(sum) = try_t(
                    &[Rational::from_integer(t0), Rational::from_integer(t1)],
                    coeffs,
                ) {
                    if best.map_or(true, |b| sum < b) {
                        best = Some(sum);
                    }
                }
            }
        }
    } else {
        // d == 3
        for t0 in lo_int[0]..=hi_int[0] {
            for t1 in lo_int[1]..=hi_int[1] {
                for t2 in lo_int[2]..=hi_int[2] {
                    if let Some(sum) = try_t(
                        &[
                            Rational::from_integer(t0),
                            Rational::from_integer(t1),
                            Rational::from_integer(t2),
                        ],
                        coeffs,
                    ) {
                        if best.map_or(true, |b| sum < b) {
                            best = Some(sum);
                        }
                    }
                }
            }
        }
    }
    
    best.expect("No integer non-negative solution found")
}

fn try_t(t: &[Rational], coeffs: &[Vec<Rational>]) -> Option<i64> {
    let d = t.len();
    let mut total: i64 = 0;
    
    for a in coeffs {
        let mut v = a[d].clone(); // constant term
        for k in 0..d {
            v = v + a[k].clone() * t[k].clone();
        }
        
        if v < Rational::zero() {
            return None;
        }
        
        if !v.is_integer() {
            return None;
        }
        
        total += v.to_integer();
    }
    
    Some(total)
}

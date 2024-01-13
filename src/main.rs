// Implementing DPLL algorithm for SAT solvability
// reference - https://sat.inesc-id.pt/~ines/sac10.pdf
// DPLL is a complete algorithm for SAT solvability, meaning that it will always find a solution if one exists. on the other hand local search algorithms are incomplete, meaning that they may not find a solution even if one exists.

// Importing the HashMap from the std library.
use std::collections::HashMap;

// Define a type alias 'Assignment' to represent variable assignments (char -> bool).
type Assignment = HashMap<char, bool>;

// Function: dpll_solve
// Parameters:
//   - cnf: A reference to a vector of vectors representing the Conjunctive Normal Form (CNF).
//   - assignment: A mutable reference to the Assignment HashMap for storing variable assignments.
// Returns: A boolean indicating whether a satisfying assignment is found.
fn dpll_solve(cnf: &Vec<Vec<i32>>, assignment: &mut Assignment) -> bool {
    if cnf.is_empty() {
        // All clauses satisfied, solution found
        return true;
    }

    if cnf.iter().any(|clause| clause.is_empty()) {
        // Found an empty clause, indicating a conflict; backtrack
        return false;
    }

    // Choose a variable from the first clause in the CNF.
    println!("Current CNF: {:?}", cnf);
    let variable = cnf[0][0].abs() as usize;
    println!("Choosing variable {}", (variable as u8 + b'A' - 1) as char);

    // Try assigning true
    assignment.insert((variable as u8 + b'A' - 1) as char, true);
    let reduced_cnf = reduce(cnf, variable, true);
    println!("Reduced CNF: {:?}", reduced_cnf);

    if dpll_solve(&reduced_cnf, assignment) {
        return true;
    }

    // If not successful, backtrack
    assignment.remove(&((variable as u8 + b'A' - 1) as char));

    // Try assigning false
    assignment.insert((variable as u8 + b'A' - 1) as char, false);
    let reduced_cnf = reduce(cnf, variable, false);

    if dpll_solve(&reduced_cnf, assignment) {
        return true;
    }

    // If not successful, undo assignment and backtrack
    assignment.remove(&((variable as u8 + b'A' - 1) as char));

    false
}

// Function: reduce
// Parameters:
//   - cnf: A reference to a vector of vectors representing the Conjunctive Normal Form (CNF).
//   - variable: The chosen variable for reduction.
//   - value: The truth value assigned to the variable.
// Returns: A vector of vectors representing the reduced CNF.
fn reduce(cnf: &Vec<Vec<i32>>, variable: usize, value: bool) -> Vec<Vec<i32>> {
    // Filter out clauses containing the chosen variable with the assigned value.
    cnf.iter()
        .filter(|clause| {
            !clause.contains(&(variable as i32 * if value { 1 } else { -1 })) &&
            !clause.contains(&(variable as i32 * if !value { 1 } else { -1 }))
        })
        .cloned()
        .collect()
}


fn parse_cnf(expression: &str) -> Vec<Vec<i32>> {
    todo!("Parse the input string into a CNF formula {}", expression);
}

fn main() {
    // Example formula: (A || B) && (!A || C)
    println!("Solving (A || B) && (!A || C):");
    let cnf = vec![vec![1, 2], vec![-1, 3]];
    let mut assignment = Assignment::new();

    if dpll_solve(&cnf, &mut assignment) {
        println!("SAT Solution found:");
        for (variable, value) in &assignment {
            println!("{}: {}", variable, value);
        }
    } else {
        println!("No SAT solution found.");
    }
}

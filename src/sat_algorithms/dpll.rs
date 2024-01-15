use crate::translators::naiveconverter::char_to_i32;
use std::collections::{HashMap};// Define a type alias 'Assignment' to represent variable assignments (char -> bool).
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
pub type Assignment = HashMap<char, bool>;

// Implementing DPLL algorithm for SAT solvability
// reference - https://sat.inesc-id.pt/~ines/sac10.pdf
// DPLL is a complete algorithm for SAT solvability, meaning that it will always find a solution if one exists. on the other hand local search algorithms are incomplete, meaning that they may not find a solution even if one exists.

// Function: dpll_solve
// Parameters:
//   - cnf: A reference to a vector of vectors representing the Conjunctive Normal Form (CNF).
//   - assignment: A mutable reference to the Assignment HashMap for storing variable assignments.
// Returns: A boolean indicating whether a satisfying assignment is found.
pub fn dpll_solve(cnf: &mut Vec<Vec<i32>>, assignment: &mut Assignment) -> bool {
    let variables: Vec<_> = cnf
        .iter()
        .flat_map(|clause| {
            clause
                .iter()
                .map(|&literal| (literal.abs() as u8) as char)
                .filter(|var| !assignment.contains_key(var))
        })
        .collect();

    let return_value = AtomicBool::new(false);
    let assignment_mutex = Arc::new(Mutex::new(assignment));

    // Parallelize the loop over variables
    variables.par_iter().for_each(|&variable| {
        let mut cnf_clone = cnf.clone();
        let assignment_mutex = Arc::clone(&assignment_mutex);
        if internal_dpll(&mut cnf_clone, &mut assignment_mutex.lock().unwrap(), variable) {
            return_value.store(true, Ordering::Relaxed);
        }
    });

    let return_value = return_value.load(Ordering::Relaxed);

    return_value
}


// Function: reduce
// Parameters:
//   - cnf: A reference to a vector of vectors representing the Conjunctive Normal Form (CNF).
//   - variable: The chosen variable for reduction.
//   - value: The truth value assigned to the variable.
// Returns: A vector of vectors representing the reduced CNF.
fn reduce(cnf: &Vec<Vec<i32>>, variable: usize, value: bool) -> Vec<Vec<i32>> {
    // Calculate the literal for the chosen variable based on the assigned value.
    let literal = variable as i32 * if value { 1 } else { -1 };

    // Construct a new CNF without the clauses containing the chosen variable with the assigned value or its negation.
    let mut reduced_cnf: Vec<Vec<i32>> = Vec::new();

    for clause in cnf.iter() {
        let mut reduced_clause: Vec<i32> = Vec::new();
        for &l in clause.iter() {
            if l != literal && l != -literal {
                reduced_clause.push(l);
            }
        }
        if !reduced_clause.is_empty() {
            reduced_cnf.push(reduced_clause);
        }
    }

    reduced_cnf
}


fn internal_dpll(mut cnf: &mut Vec<Vec<i32>>, assignment: &mut Assignment, variable: char) -> bool {
    println!("Current assignment: {:?}", assignment);
    println!("Current variable: {:?}", variable);
    println!("Current CNF: {:?}", cnf);

    // Check if the assignment is complete
    if cnf.is_empty() {
        // All clauses satisfied, solution found
        println!("Satisfying assignment: {:?}", assignment);
        return true;
    }

    if cnf.iter().any(|clause| clause.is_empty()) {
        // Found an empty clause, indicating a conflict; backtrack
        return false;
    }

    // Check if variable is in the assignment, if yes, then return true
    if assignment.contains_key(&variable) {
        println!("Variable already assigned: {:?}", variable);
        return assignment[&variable];
    }

    // Try assigning true
    println!("Trying to assign true to {:?}", variable);
    assignment.insert(variable, true);
    let mut reduced_cnf = reduce(&cnf.clone(), char_to_i32(variable) as usize, true);

    println!("Reduced CNF: {:?}", reduced_cnf);

    // Check if the reduced CNF is satisfiable
    if internal_dpll(&mut reduced_cnf, assignment, variable) {
        return true;
    } else {
        println!("Backtracking...");
    }

    // If not successful, backtrack
    assignment.remove(&variable);

    // Try assigning false
    println!("Trying to assign false to {:?}", variable);
    assignment.insert(variable, false);
    let mut reduced_cnf = reduce(&cnf.clone(), char_to_i32(variable) as usize, true);
    println!("Reduced CNF: {:?}", reduced_cnf);

    // Check if the reduced CNF is satisfiable
    if internal_dpll(&mut reduced_cnf, assignment, variable) {
        return true;
    } else {
        println!("Backtracking...");
    }

    // If not successful, undo assignment and backtrack
    assignment.remove(&variable);

    false
}
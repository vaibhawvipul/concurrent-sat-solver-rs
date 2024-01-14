mod sat_algorithms;
use sat_algorithms::dpll::{dpll_solve, Assignment};

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

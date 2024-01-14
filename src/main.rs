mod sat_algorithms;
mod translators;
use sat_algorithms::dpll::{Assignment, dpll_solve};
use translators::cnfconverter::{BoolExpr, to_cnf};

fn main() {
    // Example formula: (A || B) && (!A || C)
    let bool_expr = BoolExpr::Or(
        Box::new(BoolExpr::And(
            Box::new(BoolExpr::Variable('A')),
            Box::new(BoolExpr::Variable('B')),
        )),
        Box::new(BoolExpr::And(
            Box::new(BoolExpr::Not(Box::new(BoolExpr::Variable('A')))),
            Box::new(BoolExpr::Variable('C')),
        )),
    );

    // let bool_expr = BoolExpr::And(
    //     Box::new(BoolExpr::Or(
    //         Box::new(BoolExpr::Variable('A')),
    //         Box::new(BoolExpr::Variable('B')),
    //     )),
    //     Box::new(BoolExpr::Or(
    //         Box::new(BoolExpr::Variable('C')),
    //         Box::new(BoolExpr::Variable('D')),
    //     )),
    // );

    println!("solving for Boolean expression: {:?}", bool_expr);

    // Convert the boolean expression to CNF
    let cnf = to_cnf(&bool_expr);

    // Print the CNF representation
    println!("CNF representation: {:?}", cnf);
        
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

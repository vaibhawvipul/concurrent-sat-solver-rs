// Define the boolean expression syntax tree
#[derive(Debug)]
pub enum BoolExpr {
    Variable(char),
    Not(Box<BoolExpr>),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
}

pub fn char_to_i32(ch: char) -> i32 {
    ch as i32
}

// Naive translation to CNF
pub fn to_cnf(expr: &BoolExpr) -> Vec<Vec<i32>> {
    match expr {
        BoolExpr::Variable(var) => vec![vec![char_to_i32(*var)]],
        BoolExpr::Not(inner) => negate_cnf(&to_cnf(inner)),
        BoolExpr::And(left, right) => combine_and_cnf(&to_cnf(left), &to_cnf(right)),
        BoolExpr::Or(left, right) => combine_or_cnf(&to_cnf(left), &to_cnf(right)),
    }
}

// Helper function to negate a CNF
fn negate_cnf(cnf: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    cnf.iter().map(|clause| {
        clause.iter().map(|lit| -lit).collect::<Vec<i32>>()
    }).collect()
}

// Helper function to combine CNFs with AND
fn combine_and_cnf(left: &Vec<Vec<i32>>, right: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    result.extend_from_slice(left);
    result.extend_from_slice(right);

    println!("and result: {:?}", result);
    result
}

// Helper function to combine CNFs with OR
fn combine_or_cnf(left: &Vec<Vec<i32>>, right: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for clause_left in left.iter() {
        for clause_right in right.iter() {
            let mut combined_clause = Vec::new();
            combined_clause.extend_from_slice(clause_left);
            combined_clause.extend_from_slice(clause_right);
            result.push(combined_clause);
        }
    }

    println!("or result: {:?}", result);
    result
}



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
    left.iter().flat_map(|left_clause| {
        right.iter().map(move |right_clause| {
            left_clause.iter().cloned().chain(right_clause.iter().cloned()).collect()
        })
    }).collect()
}

// Helper function to combine CNFs with OR
fn combine_or_cnf(left: &Vec<Vec<i32>>, right: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    left.iter().chain(right.iter()).cloned().collect()
}


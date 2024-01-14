// Define a type for variables in the expression
type Variable = String;

// Define a type for the syntax tree of the boolean expression
enum Expr {
    Variable(Variable),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

pub fn process_token() {
    todo!();
}

pub fn parse_expression() {
    todo!();
}

pub fn convert_to_cnf() {
    todo!();
}
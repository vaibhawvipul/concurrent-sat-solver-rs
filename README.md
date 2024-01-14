# concurrent-sat-solver-rs

This project is an implementation of a Concurrent DPLL (Davis–Putnam–Logemann–Loveland) SAT solver in Rust. The DPLL algorithm is used for solving the Boolean Satisfiability Problem (SAT), and the concurrent implementation leverages Rust's parallel programming capabilities to enhance its performance.

## Features

- **Concurrent Execution:** Utilizes parallelization to explore different branches of the solution space concurrently, improving solving efficiency on multi-core processors.

## How to Use

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/vaibhawvipul/concurrent-sat-solver-rs.git
   ```

2. **Navigate to the Project:**
   ```bash
   cd concurrent-sat-solver-rs
   ```

3. **Build and Run:**
   ```bash
   cargo run
   ```

## References
- [DPLL Algorithm](https://en.wikipedia.org/wiki/DPLL_algorithm)
- [Boolean Satisfiability Problem](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem)
- https://www.cs.princeton.edu/courses/archive/fall21/cos326/lec/23-01-sat.pdf
- https://www.cs.cmu.edu/~mheule/15816-f21/slides/practice.pdf
- https://sat.inesc-id.pt/~ines/sac10.pdf
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
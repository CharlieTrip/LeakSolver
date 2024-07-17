pub mod aes_solver;

// use indextree::IndexTree;
// use std::time::Duration;

// /// Solver Struct
// /// X: input, K: key, I: leak input, L: leak output
// pub struct Solver<X, K, I, L> {
//   inputs: Vec<X>,
//   leaks: Vec<Vec<L>>,
//   leakfun: fn(I) -> L,
//   index: IndexTree,
//   solutions: Vec<K>,
// }

// /// Solver Trait
// /// X: input, K: key, I: leak input, L: leak output
// pub trait Solving<X, K, I, L> {
//   /// Generate solver for the specific problem
//   fn new(inputs: Vec<X>, leaks: Vec<L>, leakfun: fn(I) -> L) -> Solver<X, K, I, L>;

//   /// Get the key candidates for the solver's problem
//   fn get_candidates(solver: Solver<X, K, I, L>) -> Vec<K>;

//   /// Solve the problem
//   /// Return the candidates and the time spent computing
//   fn solve(&self) -> (Vec<K>, Duration);
// }

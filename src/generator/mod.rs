pub mod aes_generator;

/// Generator trait
/// X: input, K: key, I: leak input, L: leak output
pub trait Generator<X, K, I, L> {
  /// Generate leaks
  fn generate(input: &X, key: &K, leakfun: Box<dyn Fn(&I) -> L>) -> Vec<L>;
}

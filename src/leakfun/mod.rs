pub mod hw8;

/// Describe the trait of a leaking function.
/// I: input, O: output
pub trait LeakFun<I, O> {
  // type LeakF = fn(I) -> O;
  // type LeakFInv = fn(O) -> Vec<I>;

  /// Leak function
  fn leak_f(input: I) -> O;

  // /// Wrapper of `leak_f` to get a closure
  // fn get_leak_f() -> Self::LeakF;

  /// Return counter-image of a leak
  fn inv_leak_f(leak: O) -> Vec<I>;

  // /// Wrapper of `inv_leak_f` to get a closure
  // fn get_inv_leak_f() -> Self::LeakFInv;
}

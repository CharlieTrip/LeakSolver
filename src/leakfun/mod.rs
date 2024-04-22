pub mod hw8;

/// Describe the trait of a leaking function.
/// I: input, O: output
pub trait LeakFun<I, O> {
  /// Leak function
  fn leak_f(input: I) -> O
  where
    Self: Sized;

  /// Wrapper of `leak_f` to get a closure
  fn get_leak_f(&self) -> Box<dyn Fn(&I) -> O>;

  /// Return counter-image of a leak
  fn inv_leak_f(leak: O) -> Vec<I>
  where
    Self: Sized;

  /// Wrapper of `inv_leak_f` to get a closure
  fn get_inv_leak_f(&self) -> Box<dyn Fn(&O) -> Vec<I>>;
}

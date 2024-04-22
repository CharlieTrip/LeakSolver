#[cfg(test)]
extern crate leak_solver;

mod hw_leak_test {
  use crate::leak_solver::leakfun::hw8::Hamming8 as H8;
  use crate::leak_solver::leakfun::LeakFun;

  #[test]
  fn hw() {
    for i in 0..256 {
      let w = H8::leak_f(i as u8);
      let l = H8::inv_leak_f(w);
      assert_eq!(l.contains(&(i as u8)), true);
    }
  }
}

#[cfg(test)]
extern crate leak_solver;

mod aes_solver_test {
  use crate::leak_solver::generator::aes_generator::AESGenerator2Rounds as AESGen;
  use crate::leak_solver::generator::Generator;
  use crate::leak_solver::leakfun::hw8::Hamming8;
  use crate::leak_solver::leakfun::LeakFun;
  use crate::leak_solver::solver::aes_solver::AESSolver;

  #[test]
  fn solver_single() {
    for t in 0..256 {
      let i = t as u8;
      let x: Vec<u8> = [i; 16].to_vec();
      let k: Vec<u8> = [82 ^ i; 16].to_vec();

      let lf = Hamming8::leak_f;
      let ilf = Hamming8::inv_leak_f;

      let gen = AESGen::generate(&x, &k, lf);
      let mut solver = AESSolver::new(&([x].to_vec()), &([gen].to_vec()), (lf, ilf));

      let sols = solver.solve();
      assert_eq!(sols.contains(&k), true);
    }
  }

  #[test]
  fn solver_double() {
    for t in 0..256 {
      let i = t as u8;
      let x1: Vec<u8> = [10 ^ i; 16].to_vec();
      let x2: Vec<u8> = [i; 16].to_vec();
      let k: Vec<u8> = [82 ^ i; 16].to_vec();

      let lf = Hamming8::leak_f;
      let ilf = Hamming8::inv_leak_f;
      let gen1 = AESGen::generate(&x1, &k, lf);
      let gen2 = AESGen::generate(&x2, &k, lf);

      let mut solver = AESSolver::new(&([x1, x2].to_vec()), &([gen1, gen2].to_vec()), (lf, ilf));

      let sols = solver.solve();
      assert_eq!(sols.contains(&k), true);
    }
  }
}

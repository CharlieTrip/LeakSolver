#[cfg(test)]
extern crate leak_solver;

mod aes_gen_h8_test {
  use crate::leak_solver::generator::aes_generator::AESGenerator2Rounds as AESGen;
  use crate::leak_solver::generator::Generator;
  use crate::leak_solver::leakfun::hw8::Hamming8 as H8;
  use crate::leak_solver::leakfun::LeakFun;

  fn leak() -> Box<dyn Fn(&u8) -> u8> {
    let h8 = H8 {};
    let lf = h8.get_leak_f();
    lf
  }

  #[test]
  fn generation() {
    let lf = leak();

    let k: Vec<u8> = [82; 16].to_vec();
    let x: Vec<u8> = [0; 16].to_vec();

    let sol: Vec<u8> = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 5, 4, 4, 4, 6, 0, 0, 0, 5, 4, 4,
      4,
    ]
    .to_vec();
    let gen = AESGen::generate(&x, &k, lf);

    assert_eq!(sol, gen);
  }
}

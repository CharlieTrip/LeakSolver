#[cfg(test)]
extern crate leak_solver;

mod aes_gen_h8_test {
  use crate::leak_solver::generator::aes_generator::AESGenerator2Rounds as AESGen;
  use crate::leak_solver::generator::Generator;
  use crate::leak_solver::leakfun::LeakFun;
  use leak_solver::leakfun::hw8::Hamming8;

  fn leak() -> fn(u8) -> u8 {
    Hamming8::leak_f
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

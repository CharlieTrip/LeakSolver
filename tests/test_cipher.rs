#[cfg(test)]
extern crate leak_solver;

mod aes_test {
  use leak_solver::cipher::aes::AES;

  #[test]
  fn s() {
    for i in 0..256 {
      assert_eq!(AES::s(i as u8), AES::AES_SBOX[i]);
      assert_eq!(AES::si(i as u8), AES::INVERSE_AES_SBOX[i]);
      assert_eq!(AES::si(AES::s(i as u8)), i as u8);
    }
  }

  #[test]
  fn m2() {
    for i in 0..256 {
      assert_eq!(AES::m2(i as u8), AES::M2[i]);
    }
  }

  #[test]
  fn m3() {
    for i in 0..256 {
      assert_eq!(AES::m3(i as u8), AES::M3[i]);
    }
  }
}

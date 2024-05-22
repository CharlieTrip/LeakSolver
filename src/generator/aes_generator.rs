use crate::cipher::aes::AES;
use crate::generator::Generator;

/// AES Generator
/// X: input, K: key, I: leak input, L: leak output

pub struct AESGenerator2Rounds {}

impl Generator<Vec<u8>, Vec<u8>, u8, u8> for AESGenerator2Rounds {
  /// Generate leaks
  fn generate(input: &Vec<u8>, key: &Vec<u8>, leakfun: fn(u8) -> u8) -> Vec<u8> {
    let k2 = AES::key2(&key);
    let mut s: Vec<u8>;
    s = AES::xor(&input, &key);
    s = AES::sbox(&s);
    let mut w1: Vec<u8> = s.iter().map(|x| leakfun(*x)).collect();
    s = AES::sbox(&AES::xor(&AES::ml(&s), &k2));
    let mut w2: Vec<u8> = s.iter().map(|x| leakfun(*x)).collect();

    w1.append(&mut w2);
    w1
  }
}

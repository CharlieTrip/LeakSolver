use crate::leakfun::LeakFun;

#[derive(Copy, Clone)]
pub struct Hamming8 {}

/// # 8-bit Hamming Weight Leakage Model
/// *AAA:* Strict usage of u8!

impl LeakFun<u8, u8> for Hamming8 {
  // type LeakF = fn(u8) -> u8;
  // type LeakFInv = fn(u8) -> Vec<u8>;

  fn leak_f(input: u8) -> u8 {
    input.count_ones() as u8
  }

  fn inv_leak_f(weight: u8) -> Vec<u8> {
    match weight {
      0 => W0.to_vec(),
      1 => W1.to_vec(),
      2 => W2.to_vec(),
      3 => W3.to_vec(),
      4 => W4.to_vec(),
      5 => W5.to_vec(),
      6 => W6.to_vec(),
      7 => W7.to_vec(),
      8 => W8.to_vec(),
      _ => WT.to_vec(),
    }
  }

  // fn get_leak_f() -> Self::LeakF {
  //   Hamming8::leak_f
  // }

  // fn get_inv_leak_f() -> Self::LeakFInv {
  //   Hamming8::inv_leak_f
  // }
}

/// Pre-computed leaks pre-images
static WT: [u8; 256] = [
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
  51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
  75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
  99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
  118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136,
  137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155,
  156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174,
  175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
  194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212,
  213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231,
  232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250,
  251, 252, 253, 254, 255,
];
static W0: [u8; 1] = [0];
static W1: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
static W2: [u8; 28] = [
  3, 5, 6, 9, 10, 12, 17, 18, 20, 24, 33, 34, 36, 40, 48, 65, 66, 68, 72, 80, 96, 129, 130, 132,
  136, 144, 160, 192,
];
static W3: [u8; 56] = [
  7, 11, 13, 14, 19, 21, 22, 25, 26, 28, 35, 37, 38, 41, 42, 44, 49, 50, 52, 56, 67, 69, 70, 73,
  74, 76, 81, 82, 84, 88, 97, 98, 100, 104, 112, 131, 133, 134, 137, 138, 140, 145, 146, 148, 152,
  161, 162, 164, 168, 176, 193, 194, 196, 200, 208, 224,
];
static W4: [u8; 70] = [
  15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86, 89, 90,
  92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149, 150, 153, 154,
  156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201, 202, 204, 209, 210,
  212, 216, 225, 226, 228, 232, 240,
];
static W5: [u8; 56] = [
  31, 47, 55, 59, 61, 62, 79, 87, 91, 93, 94, 103, 107, 109, 110, 115, 117, 118, 121, 122, 124,
  143, 151, 155, 157, 158, 167, 171, 173, 174, 179, 181, 182, 185, 186, 188, 199, 203, 205, 206,
  211, 213, 214, 217, 218, 220, 227, 229, 230, 233, 234, 236, 241, 242, 244, 248,
];
static W6: [u8; 28] = [
  63, 95, 111, 119, 123, 125, 126, 159, 175, 183, 187, 189, 190, 207, 215, 219, 221, 222, 231, 235,
  237, 238, 243, 245, 246, 249, 250, 252,
];
static W7: [u8; 8] = [127, 191, 223, 239, 247, 251, 253, 254];
static W8: [u8; 1] = [255];

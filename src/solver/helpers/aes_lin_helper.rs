// #![allow(unused_imports, unused_variables, dead_code)]

use crate::cipher::aes::AES;
use crate::solver::Helper;
use crate::solver::InputHelper;
use crate::solver::LinearHelper;

use tree_jump::Constrain;

type X = Vec<u8>;
type K = Vec<u8>;

type I = u8;
type L = u8;
type LeakF = fn(I) -> L;
type LeakFInv = fn(u8) -> Vec<u8>;

const PERMUTATION: [usize; 16] = [0, 5, 10, 15, 13, 2, 1, 14, 3, 12, 9, 4, 7, 8, 6, 11];
const INV_PERMUTATION: [usize; 16] = [0, 6, 5, 8, 11, 1, 14, 12, 13, 10, 2, 15, 9, 4, 7, 3];
const SKIPS: [usize; 9] = [5, 6, 8, 10, 12, 13, 14, 15, 16];

#[derive(Clone, Debug)]
pub struct AESHelper {
  inputs: AESInputHelper,
  pub c: u8,
  pub mask: Vec<usize>,
  pub index: usize,
}

impl Helper for AESHelper {}
impl AESHelper {
  pub fn new(inputs: AESInputHelper, c: u8, mask: Vec<usize>, index: usize) -> Self {
    AESHelper {
      inputs,
      c,
      mask,
      index,
    }
  }
}

#[derive(Clone, Debug)]
pub struct AESInputHelper {
  pub x: Vec<X>,
  pub w: Vec<Vec<L>>,
  pub candidates: Vec<K>,
  pub leakfun: LeakF,
}

impl AESInputHelper {
  pub fn new(inputs: Vec<X>, leaks: Vec<Vec<L>>, leakfun: (LeakF, LeakFInv)) -> Self {
    let dim = inputs.len();
    let mut leakss: Vec<Vec<L>> = vec![];

    for l in (*leaks).iter().map(|ll| ll.chunks(16)).into_iter() {
      for li in l {
        leakss.push((*li).to_vec().clone());
      }
    }

    // Compute Intersected Key Candidate List
    let mut candidates = AESInputHelper::get_candidates(&inputs[0], &leakss[0], leakfun.1);
    let mut weights: Vec<Vec<L>> = vec![leakss[1].clone()];

    for i in 1..dim {
      let input = &inputs[i];
      let leak1 = &leakss[2 * i];
      let leak2 = leakss[(2 * i) + 1].clone();
      weights.push(leak2);
      let cands = AESInputHelper::get_candidates(&input, &leak1, leakfun.1);
      // TODO: Find better way to do intersection
      for i in 0..16 {
        let mut tmp = vec![];
        for c in cands[i].iter() {
          if candidates[i].contains(c) {
            tmp.push(*c);
          }
        }
        candidates[i] = tmp.clone();
      }
    }
    let candidates: Vec<Vec<u8>> = PERMUTATION
      .iter()
      .map(|x| candidates[*x as usize].clone())
      .collect();

    AESInputHelper {
      x: inputs,
      w: weights,
      candidates: candidates,
      leakfun: leakfun.0,
    }
  }

  /// Get the key candidates for an input, leaks and leak function
  pub fn get_candidates(x: &X, w1: &Vec<L>, leakfuninv: LeakFInv) -> Vec<K> {
    let binding = (*w1).iter().map(|i| leakfuninv(*i)).into_iter();
    let binding = binding.zip((*x).iter()).collect::<Vec<_>>();
    let cw1: Vec<_> = binding
      .iter()
      .map(|(m, xi)| m.iter().map(|y| AES::si(*y) ^ **xi).collect::<Vec<_>>())
      .collect();
    cw1
  }

  /// 2nd pre-sbox general check
  pub fn check(x: &Vec<u8>, k: &Vec<u8>, c: u8, w: u8, leakfun: LeakF) -> bool {
    if (k.len() < 5) | (x.len() < 4) {
      return false;
    }
    let t = AES::m2(AES::s(x[0] ^ k[0]))
      ^ AES::m3(AES::s(x[1] ^ k[1]))
      ^ AES::s(x[2] ^ k[2])
      ^ AES::s(x[3] ^ k[3])
      ^ AES::s(k[4])
      ^ c
      ^ k[5..].iter().fold(0, |x, y| x ^ y);
    w == leakfun(AES::s(t))
  }
}

impl InputHelper<K, AESHelper> for AESInputHelper {
  fn phi(k: Vec<u8>, input: &Option<AESHelper>) -> bool {
    match input {
      Some(help) => {
        let hh = help;
        for j in 0..hh.inputs.x.len() {
          let ks: Vec<u8> = hh.mask.iter().map(|i| k[*i]).collect();
          let xs: Vec<u8> = hh.mask[..4]
            .iter()
            .map(|i| hh.inputs.x[j][PERMUTATION[*i]])
            .collect();

          let t = Self::check(&xs, &ks, hh.c, hh.inputs.w[j][hh.index], hh.inputs.leakfun);

          if !t {
            return false;
          }
        }
        return true;
      }
      None => false,
    }
  }

  fn finalise(&self, unpermuted: &Vec<K>) -> Vec<K> {
    unpermuted
      .into_iter()
      .map(|sol| INV_PERMUTATION.to_vec().iter().map(|&i| sol[i]).collect())
      .collect()
  }

  fn candidates(&self) -> Vec<K> {
    self.candidates.clone()
  }
}

impl LinearHelper<L, AESHelper> for AESInputHelper {
  fn linear(&self) -> (Vec<Option<AESHelper>>, Vec<Constrain<u8, AESHelper>>) {
    let checksperm = vec![
      vec![0, 1, 2, 3, 4, 0],
      vec![2, 3, 0, 1, 3, 5],
      vec![1, 2, 3, 0, 7, 6],
      vec![3, 0, 1, 2, 9, 8],
      vec![11, 10, 7, 8, 4, 0, 11],
      vec![10, 7, 8, 11, 7, 6, 1],
      vec![8, 11, 10, 7, 9, 8, 12],
      vec![13, 4, 5, 12, 4, 0, 11, 13],
      vec![4, 5, 12, 13, 7, 6, 1, 10],
      vec![7, 8, 11, 10, 3, 5, 14],
      vec![5, 12, 13, 4, 3, 5, 14, 2],
      vec![12, 13, 4, 5, 9, 8, 12, 15],
      vec![9, 6, 14, 15, 4, 0, 11, 13, 9],
      vec![6, 14, 15, 9, 7, 6, 1, 10, 4],
      vec![14, 15, 9, 6, 3, 5, 14, 2, 7],
      vec![15, 9, 6, 14, 9, 8, 12, 15, 3],
    ];
    let rc = [
      AES::rc(1),
      0,
      0,
      0,
      AES::rc(1),
      0,
      0,
      AES::rc(1),
      0,
      0,
      0,
      0,
      AES::rc(1),
      0,
      0,
      0,
    ];
    let checkindex = [0, 2, 1, 3, 4, 5, 7, 8, 9, 6, 10, 11, 12, 13, 14, 15];
    let skipindex: Vec<usize> = [1, 2, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9, 9, 9, 9, 9]
      .iter()
      .map(|&i| SKIPS[i - 1] - 1)
      .collect();

    let helpers: Vec<Option<AESHelper>> = (0..16)
      .into_iter()
      .map(|i| {
        Some(AESHelper::new(
          self.clone(),
          rc[i],
          checksperm[i].clone(),
          checkindex[i],
        ))
      })
      .collect::<Vec<Option<AESHelper>>>();

    let phis: Vec<Constrain<u8, AESHelper>> = (0..16)
      .into_iter()
      .map(|i| Constrain {
        index: skipindex[i],
        constrain: AESInputHelper::phi,
      })
      .collect();

    (helpers, phis)
  }
}

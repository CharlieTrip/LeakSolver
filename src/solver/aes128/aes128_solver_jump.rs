#![allow(dead_code)]
//#![allow(unused_assignments, unused_variables, unused_imports)]

use crate::cipher::aes::AES;
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use tree_jump::Constrain;

use std::time::Duration;
use tree_jump::TreeJump;

type X = Vec<u8>;
type K = Vec<u8>;
type I = u8;
type L = u8;

type LeakF = fn(I) -> L;
type LeakFInv = fn(L) -> Vec<I>;

pub struct AESSolverJump {
  pub inputs: Vec<Option<Helper>>,
  pub leaks: Vec<Vec<L>>,
  candidates: Vec<Vec<Vec<u8>>>,
  pub leakfun: (LeakF, LeakFInv),
  pub index: TreeJump<u8, Helper>,
  pub solutions: Vec<K>,
  bars: MultiProgress,
}

/// Solver Trait
/// X: input, K: key, I: leak input, L: leak output
impl<'a> AESSolverJump {
  const PERMUTATION: [usize; 16] = [0, 5, 10, 15, 13, 2, 1, 14, 3, 12, 9, 4, 7, 8, 6, 11];
  const INV_PERMUTATION: [usize; 16] = [0, 6, 5, 8, 11, 1, 14, 12, 13, 10, 2, 15, 9, 4, 7, 3];
  const SKIPS: [usize; 9] = [5, 6, 8, 10, 12, 13, 14, 15, 16];

  /// Generate solver for the specific problem
  /// TODO: sanitize dimensions
  pub fn new(inputs: &Vec<X>, leaks: &Vec<Vec<L>>, leakfun: (LeakF, LeakFInv)) -> AESSolverJump {
    let dim = inputs.len();
    let mut leakss: Vec<Vec<L>> = vec![];

    for l in (*leaks).iter().map(|ll| ll.chunks(16)).into_iter() {
      for li in l {
        leakss.push((*li).to_vec().clone());
      }
    }

    // Compute Intersected Key Candidate List
    let mut candidates = AESSolverJump::get_candidates(&inputs[0], &leakss[0], leakfun.1);
    let mut weights: Vec<Vec<L>> = vec![leakss[1].clone()];

    for i in 1..dim {
      let input = &inputs[i];
      let leak1 = &leakss[2 * i];
      let leak2 = leakss[(2 * i) + 1].clone();
      weights.push(leak2);
      let cands = AESSolverJump::get_candidates(&input, &leak1, leakfun.1);
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

    let candidates: Vec<Vec<u8>> = Self::PERMUTATION
      .iter()
      .map(|x| candidates[*x as usize].clone())
      .collect();
    // let _dimensions: Vec<_> = candidates.iter().map(|cand| cand.len()).collect();
    let solutions: Vec<K> = vec![];

    let candidates: Vec<Vec<Vec<u8>>> = candidates
      .iter()
      .map(|cand| cand.iter().map(|&x| vec![x]).collect())
      .collect();

    let mp = MultiProgress::new();

    let pb = ProgressBar::new(1);
    let sty = ProgressStyle::with_template(
            "{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] ({pos}/{len}, {eta} ({per_sec})",
        )
        .unwrap()
        .progress_chars("##-");
    pb.set_style(sty.clone());

    let pb = mp.add(pb);

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
      .map(|&i| Self::SKIPS[i - 1] - 1)
      .collect();

    let helpers: Vec<Option<Helper>> = (0..16)
      .into_iter()
      .map(|i| {
        Some(Helper::new(
          inputs.to_vec(),
          rc[i],
          weights.clone(),
          checksperm[i].clone(),
          checkindex[i],
          leakfun.0,
        ))
      })
      .collect::<Vec<Option<Helper>>>();

    let phis: Vec<Constrain<u8, Helper>> = (0..16)
      .into_iter()
      .map(|i| Constrain {
        index: skipindex[i],
        constrain: Helper::phi,
      })
      .collect();

    let treejump = TreeJump::new_vector(Some(helpers.clone()), candidates.clone(), phis, Some(pb));

    AESSolverJump {
      index: treejump,
      inputs: helpers,
      candidates: candidates,
      leaks: leakss,
      leakfun: leakfun,
      solutions: solutions.to_vec(),
      bars: mp,
    }
  }

  /// Solve the problem
  /// Return the candidates and the time spent computing
  pub fn solve(&mut self) -> Vec<Vec<u8>> {
    let sols = self.index.search();

    let sols: Vec<K> = sols
      .into_iter()
      .map(|sol| {
        Self::INV_PERMUTATION
          .to_vec()
          .iter()
          .map(|&i| sol[i])
          .collect()
      })
      .collect();

    let _ = self.bars.clear();

    self.solutions = sols.clone();
    sols
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

  pub fn timing(&self) -> Option<Duration> {
    self.index.timing()
  }

  pub fn counting(&self) -> usize {
    self.index.counting()
  }
}

#[derive(Clone, Debug)]
pub struct Helper {
  pub x: Vec<Vec<u8>>,
  pub c: u8,
  pub w: Vec<Vec<u8>>,
  pub mask: Vec<usize>,
  pub index: usize,
  pub leakfun: LeakF,
}

impl Helper {
  pub fn new(
    x: Vec<Vec<u8>>,
    c: u8,
    w: Vec<Vec<u8>>,
    mask: Vec<usize>,
    index: usize,
    leakfun: LeakF,
  ) -> Self {
    Helper {
      x,
      c,
      w,
      mask,
      index,
      leakfun,
    }
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

  pub fn phi(k: Vec<u8>, input: &Option<Helper>) -> bool {
    match input {
      Some(help) => {
        let hh = help;
        for j in 0..hh.x.len() {
          let ks: Vec<u8> = hh.mask.iter().map(|i| k[*i]).collect();
          let xs: Vec<u8> = hh.mask[..4]
            .iter()
            .map(|i| hh.x[j][AESSolverJump::PERMUTATION[*i]])
            .collect();

          let t = Self::check(&xs, &ks, hh.c, hh.w[j][hh.index], hh.leakfun);

          if !t {
            return false;
          }
        }
        return true;
      }
      None => false,
    }
  }
}

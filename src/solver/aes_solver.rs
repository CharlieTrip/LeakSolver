#![allow(unused_assignments)]

use crate::cipher::aes::AES;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use indextree::IndexTree;
use std::time::Duration;

type X = Vec<u8>;
type K = Vec<u8>;
type I = u8;
type L = u8;

type LeakF = fn(I) -> L;
type LeakFInv = fn(L) -> Vec<I>;

pub struct AESSolver {
  pub inputs: Vec<X>,
  pub leaks: Vec<Vec<L>>,
  pub candidates: Vec<Vec<u8>>,
  pub leakfun: (LeakF, LeakFInv),
  pub index: IndexTree,
  pub solutions: Vec<K>,
}

/// Solver Trait
/// X: input, K: key, I: leak input, L: leak output
impl<'a> AESSolver {
  const PERMUTATION: [usize; 16] = [0, 5, 10, 15, 13, 2, 1, 14, 3, 12, 9, 4, 7, 8, 6, 11];
  const INV_PERMUTATION: [usize; 16] = [0, 6, 5, 8, 11, 1, 14, 12, 13, 10, 2, 15, 9, 4, 7, 3];
  const SKIPS: [usize; 9] = [6, 7, 9, 11, 13, 14, 15, 16, 17];

  /// Generate solver for the specific problem
  /// TODO: sanitize dimensions
  pub fn new(inputs: &Vec<X>, leaks: &'a Vec<Vec<L>>, leakfun: (LeakF, LeakFInv)) -> AESSolver {
    let dim = inputs.len();
    let mut leakss: Vec<Vec<L>> = vec![];

    for l in (*leaks).iter().map(|ll| ll.chunks(16)).into_iter() {
      for li in l {
        leakss.push((*li).to_vec().clone());
      }
    }

    // Compute Intersected Key Candidate List
    let mut candidates = AESSolver::get_candidates(&inputs[0], &leakss[0], leakfun.1);
    let mut weights: Vec<Vec<L>> = vec![leakss[1].clone()];

    for i in 1..dim {
      let input = &inputs[i];
      let leak1 = &leakss[2 * i];
      let leak2 = leakss[(2 * i) + 1].clone();
      weights.push(leak2);
      let cands = AESSolver::get_candidates(&input, &leak1, leakfun.1);
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

    let dimensions: Vec<_> = candidates.iter().map(|cand| cand.len()).collect();

    let nis: Vec<usize> = Self::PERMUTATION
      .iter()
      .map(|x| dimensions[*x as usize] as usize)
      .collect();

    let index = IndexTree::new(&nis.to_vec(), &(Self::SKIPS.to_vec()));
    let solutions: Vec<K> = vec![];

    AESSolver {
      index,
      inputs: inputs.to_vec(),
      candidates: candidates.to_vec(),
      leaks: leakss,
      leakfun: leakfun,
      solutions: solutions.to_vec(),
    }
  }

  /// Solve the problem
  /// Return the candidates and the time spent computing
  pub fn solve(&mut self) -> (Vec<Vec<u8>>, Duration) {
    // TODO: Fix overflow is usize is u32

    // All info for ProgressBar
    let mut dims: Vec<usize> = self.index.dimensions().clone();
    dims.push(1 as usize);
    let mut postnums: Vec<usize> = AESSolver::SKIPS
      .iter()
      .map(|s| std::iter::Product::product((&dims[(s + 0)..]).iter()))
      .collect::<Vec<usize>>();
    let prenum: usize = std::iter::Product::product((&dims[0..(AESSolver::SKIPS[1])]).iter());

    let mut n: u64 = 1;
    let mut too_big: bool = false;

    match prenum.checked_mul((postnums[1] as u64).try_into().unwrap()) {
      Some(nn) => n = nn as u64,
      None => too_big = true,
    }

    let sty = ProgressStyle::with_template(
            "{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] ({pos}/{len}, {eta} ({per_sec})",
        )
        .unwrap()
        .progress_chars("##-");

    if too_big {
      n = prenum as u64;
      postnums = vec![(dims[AESSolver::SKIPS[0] - 1]), 1, 0, 0, 0, 0, 0, 0, 0];
    }

    // TODO: Double check increases for correctness
    let pb = ProgressBar::new(n);
    pb.set_style(sty.clone());
    pb.set_message("Total ");

    if too_big {
      pb.set_message("Part. ");
    }

    let mut sol: Vec<Vec<u8>> = Vec::new();

    let mut changed: bool = true;
    let mut tmp: bool;
    let mut res: Result<bool, ()> = Ok(false);

    'general: while self.index.check() {
      // Check Eq 1
      let x: Vec<usize> = vec![0, 1, 2, 3, 4, 0];
      (_, res) = self.check_cycle(x, AES::rc(1), 0, 1, Some((&pb, postnums[0] as u64)));
      if res != Ok(false) {
        continue;
      }

      // Check Eq 3
      let x: Vec<usize> = vec![2, 3, 0, 1, 3, 5];
      (_, res) = self.check_cycle(x, 0, 2, 2, Some((&pb, postnums[1] as u64)));
      if res != Ok(false) {
        continue;
      }

      // Check Eq 2
      let x: Vec<usize> = vec![1, 2, 3, 0, 7, 6];
      (_, res) = self.check_cycle(x, 0, 1, 3, Some((&pb, postnums[2] as u64)));
      if res != Ok(false) {
        continue;
      }

      // Check Eq 4
      let x: Vec<usize> = vec![3, 0, 1, 2, 9, 8];
      (_, res) = self.check_cycle(x, 0, 3, 4, Some((&pb, postnums[3] as u64)));
      if res != Ok(false) {
        continue;
      }

      // Check Eq 5,6
      changed = true;
      while changed {
        let x: Vec<usize> = vec![11, 10, 7, 8, 4, 0, 11];
        (_, res) = self.check_cycle(x, AES::rc(1), 4, 5, Some((&pb, postnums[4] as u64)));
        if res != Ok(false) {
          continue 'general;
        }

        let x: Vec<usize> = vec![10, 7, 8, 11, 7, 6, 1];
        (changed, res) = self.check_cycle(x, 0, 5, 5, Some((&pb, postnums[4] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
      }

      // Check Eq 8
      let x: Vec<usize> = vec![8, 11, 10, 7, 9, 8, 12];
      (_, res) = self.check_cycle(x, 0, 7, 6, Some((&pb, postnums[5] as u64)));
      if res != Ok(false) {
        continue;
      }

      // Check Eq 9,10
      changed = true;
      while changed {
        let x: Vec<usize> = vec![13, 4, 5, 12, 4, 0, 11, 13];
        (_, res) = self.check_cycle(x, AES::rc(1), 8, 7, Some((&pb, postnums[6] as u64)));

        if res != Ok(false) {
          continue 'general;
        }

        let x: Vec<usize> = vec![4, 5, 12, 13, 7, 6, 1, 10];
        (changed, res) = self.check_cycle(x, 0, 9, 7, Some((&pb, postnums[6] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
      }

      // Check Eq 7,11
      changed = true;
      while changed {
        let x: Vec<usize> = vec![7, 8, 11, 10, 3, 5, 14];
        (_, res) = self.check_cycle(x, 0, 6, 8, Some((&pb, postnums[7] as u64)));
        if res != Ok(false) {
          continue 'general;
        }

        let x: Vec<usize> = vec![5, 12, 13, 4, 3, 5, 14, 2];
        (changed, res) = self.check_cycle(x, 0, 10, 8, Some((&pb, postnums[7] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
      }

      // Check Eq 12,13,14,15,16
      changed = true;
      while changed {
        let x: Vec<usize> = vec![12, 13, 4, 5, 9, 8, 12, 15];
        (tmp, res) = self.check_cycle(x, 0, 11, 9, Some((&pb, postnums[8] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
        changed = tmp;

        let x: Vec<usize> = vec![9, 6, 14, 15, 4, 0, 11, 13, 9];
        (tmp, res) = self.check_cycle(x, AES::rc(1), 12, 9, Some((&pb, postnums[8] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
        changed = changed | tmp;

        let x: Vec<usize> = vec![6, 14, 15, 9, 7, 6, 1, 10, 4];
        (tmp, res) = self.check_cycle(x, 0, 13, 9, Some((&pb, postnums[8] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
        changed = changed | tmp;

        let x: Vec<usize> = vec![14, 15, 9, 6, 3, 5, 14, 2, 7];
        (tmp, res) = self.check_cycle(x, 0, 14, 9, Some((&pb, postnums[8] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
        changed = changed | tmp;

        let x: Vec<usize> = vec![15, 9, 6, 14, 9, 8, 12, 15, 3];
        (tmp, res) = self.check_cycle(x, 0, 15, 9, Some((&pb, postnums[8] as u64)));
        if res != Ok(false) {
          continue 'general;
        }
        changed = changed | tmp;
      }

      let ii = self.index.get();
      let ckt: Vec<u8> = (0..16)
        .map(|j| self.candidates[j][ii[AESSolver::INV_PERMUTATION[j]] as usize])
        .collect();
      sol.push(ckt.clone());
      res = self.index.inc();
      pb.inc(1);
    }

    self.solutions = sol.clone();

    pb.finish_and_clear();

    (sol, pb.elapsed())
  }

  /// Abstraction: check one level
  fn check_cycle(
    &mut self,
    checks: Vec<usize>,
    rc: u8,
    weight_i: usize,
    skip_i: usize,
    pb: Option<(&ProgressBar, u64)>,
  ) -> (bool, Result<bool, ()>) {
    let mut changed = false;
    let mut t = false;
    let mut tres = Ok(false);
    let mut pb_flag = false;
    let (pb, n) = match pb {
      Some((x, y)) => {
        pb_flag = true;
        (x, y)
      }
      None => todo!(),
    };

    'outer: while (tres == Ok(false)) & (std::ops::Not::not(t)) {
      for j in 0..self.inputs.len() {
        let ii = self.index.get();

        let xs: Vec<u8> = checks[..4]
          .iter()
          .map(|i| self.inputs[j][AESSolver::PERMUTATION[*i]])
          .collect();

        let ks = checks
          .iter()
          .map(|i| self.candidates[AESSolver::PERMUTATION[*i]][ii[*i]])
          .collect();

        t = Self::check(
          xs.clone(),
          ks,
          rc,
          self.leaks[2 * j + 1][weight_i].clone(),
          self.leakfun.0,
        );

        if std::ops::Not::not(t) {
          tres = self.index.inc_skip_v(skip_i);
          if pb_flag {
            pb.inc(n);
          }
          changed = true;
          continue 'outer;
        }
      }
    }
    (changed, tres)
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
  pub fn check(x: Vec<u8>, k: Vec<u8>, c: u8, w: u8, leakfun: LeakF) -> bool {
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

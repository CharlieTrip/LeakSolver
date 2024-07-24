// #![allow(unused_imports, unused_variables, dead_code)]

use crate::solver::helpers::aes128_helper_par::AESHelper;
use crate::solver::helpers::aes128_helper_par::AESInputHelper;
use crate::solver::InputHelper;
use crate::solver::ParallelHelper;
use crate::solver::Solver;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use core::any::Any;
use std::time::Duration;

use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use tree_jump::Constrain;
use tree_jump::TreeJump;

type K = Vec<u8>;

#[derive(Clone, Debug)]
pub struct AESSolverParallel {
  pub inputs: AESInputHelper,
  index: Vec<TreeJump<u8, AESHelper>>,
  pub solutions: Vec<K>,
  bars: MultiProgress,
  timing: Option<Duration>,
  count: usize,
}

impl Solver<K, u8, AESHelper> for AESSolverParallel {
  fn new(input: (impl InputHelper<K, u8, AESHelper> + 'static)) -> Self {
    // Casting
    let input = (&input as &dyn Any)
      .downcast_ref::<AESInputHelper>()
      .expect("Failed to cast to AESInputHelper");

    // Progress Bar Init
    let mp = MultiProgress::new();

    let pb = ProgressBar::new(1);
    let sty = ProgressStyle::with_template(
            "{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] ({pos}/{len}, {eta} ({per_sec})",
        )
        .unwrap()
        .progress_chars("##-");
    pb.set_style(sty.clone());

    let (helpers, phis) = input.conditions();
    let (par, _) = input.par_dimensions();
    let (pars, _) = input.par_skips();

    let mut phisp: Vec<Vec<Constrain<u8, AESHelper>>> = vec![];
    let mut k: usize = 0;
    for i in 0..par.len() {
      let mut tmp: Vec<Constrain<u8, AESHelper>> = vec![];
      for _ in 0..par[i].len() {
        tmp.push(phis[k].clone());
        k = k + 1;
      }
      phisp.push(tmp);
    }

    let mut treejump: Vec<TreeJump<u8, AESHelper>> = vec![];
    let mut k: usize = 0;
    for i in 0..par.len() {
      let tmp = TreeJump::<u8, AESHelper>::new_vector(
        Some(
          (0..pars[i].len())
            .into_iter()
            .map(|j| helpers[k + j].clone())
            .collect(),
        ),
        input.candidates(Some(par[i].to_vec())),
        (0..pars[i].len())
          .into_iter()
          .map(|j| phis[k + j].clone())
          .collect(),
        Some(mp.add(pb.clone())),
      );
      k = k + pars[i].len();
      treejump.push(tmp);
    }

    // let treejump: Vec<TreeJump<u8, AESHelper>> = par
    //   .iter()
    //   .enumerate()
    //   .map(|(i, p)| {
    //     TreeJump::<u8, AESHelper>::new_vector(
    //       Some(p.iter().map(|&i| helpers[i].clone()).collect()),
    //       input.candidates(Some(p.to_vec())),
    //       phisp[i].clone(),
    //       Some(pb.clone()),
    //     )
    //   })
    //   .collect();

    AESSolverParallel {
      inputs: input.clone(),
      index: treejump,
      solutions: vec![],
      bars: mp,
      timing: None,
      count: 0,
    }
  }

  fn solve(&mut self) -> Vec<K> {
    let prepar: Vec<(Vec<K>, Option<Duration>, usize)> = self
      .index
      .par_iter()
      .map(|jump| {
        let mut j = jump.clone();
        let sol = j.search();
        (sol, j.timing(), j.counting())
      })
      .collect();

    let mut pre: Vec<Vec<K>> = vec![];
    for (ds, dt, dc) in &prepar {
      pre.push(ds.to_vec());
      self.add_timing(*dt);
      self.add_count(*dc);
    }

    let (_, lin) = self.inputs.par_dimensions();

    pre.extend(self.inputs.candidates(Some(lin.clone())));

    let (par, lin) = self.inputs.par_skips();

    let (helpers, phis) = self.inputs.conditions();

    let pb = ProgressBar::new(1);
    let sty = ProgressStyle::with_template(
            "{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] ({pos}/{len}, {eta} ({per_sec})",
        )
        .unwrap()
        .progress_chars("##-");
    pb.set_style(sty.clone());

    let pb = self.bars.add(pb);

    let par: usize = par
      .iter()
      .map(|p| p.len())
      .collect::<Vec<usize>>()
      .into_iter()
      .sum();

    let mut tree = TreeJump::<u8, AESHelper>::new_vector(
      Some(
        (0..lin.len())
          .into_iter()
          .map(|i| helpers[par + i].clone())
          .collect(),
      ),
      pre.clone(),
      (0..lin.len())
        .into_iter()
        .map(|i| phis[par + i].clone())
        .collect(),
      Some(pb.clone()),
    );

    let sols = tree.search();

    self.solutions = self.inputs.finalise(&sols);

    self.add_timing(tree.timing());
    self.add_count(tree.counting());

    let _ = self.bars.clear();

    self.solutions.clone()
  }

  fn timing(&self) -> Option<Duration> {
    self.timing
  }

  fn counting(&self) -> usize {
    self.count
  }
}

impl AESSolverParallel {
  fn add_timing(&mut self, dt: Option<Duration>) {
    match (self.timing, dt) {
      (None, None) => self.timing = None,
      (Some(_), None) => (),
      (None, Some(_)) => self.timing = dt,
      (Some(t), Some(dt)) => self.timing = Some(t + dt),
    }
  }

  fn add_count(&mut self, count: usize) {
    self.count += count;
  }
}

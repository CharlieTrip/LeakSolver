#![allow(unused_imports, unused_variables, dead_code)]

use crate::solver::helpers::aes128_helper_lin::AESHelper;
use crate::solver::helpers::aes128_helper_lin::AESInputHelper;
use crate::solver::InputHelper;
use crate::solver::LinearHelper;
use crate::solver::Solver;
use core::any::Any;
use std::time::Duration;

use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use tree_jump::TreeJump;

type K = Vec<u8>;

#[derive(Clone, Debug)]
pub struct AESSolverLinear {
  pub inputs: AESInputHelper,
  index: TreeJump<u8, AESHelper>,
  pub solutions: Vec<K>,
  bars: MultiProgress,
}

impl Solver<K, u8, AESHelper> for AESSolverLinear {
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

    let pb = mp.add(pb);

    let (helpers, phis) = input.conditions();

    let treejump = TreeJump::new_vector(
      Some(helpers.clone()),
      input.candidates(None),
      phis,
      Some(pb),
    );

    AESSolverLinear {
      inputs: input.clone(),
      index: treejump,
      solutions: vec![],
      bars: mp,
    }
  }

  fn solve(&mut self) -> Vec<K> {
    let sols = self.index.search();

    let sols: Vec<K> = self.inputs.finalise(&sols);

    let _ = self.bars.clear();

    self.solutions = sols.clone();

    sols
  }

  fn timing(&self) -> Option<Duration> {
    self.index.timing()
  }

  fn counting(&self) -> usize {
    self.index.counting()
  }
}

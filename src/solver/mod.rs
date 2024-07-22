// AES Solvers
pub mod aes_solver;
pub mod aes_solver_jump;
pub mod aes_solver_old;

// Helpers
pub mod helpers;

// Solver Traits
use std::time::Duration;
use tree_jump::Constrain;

pub trait InputHelper<K, H: Helper> {
  fn phi(candidate: K, helper: &Option<H>) -> bool
  where
    Self: Sized;

  fn finalise(&self, unpermuted: &Vec<K>) -> Vec<K>
  where
    Self: Sized;

  fn candidates(&self) -> Vec<K>;
}

pub trait LinearHelper<K, H: Helper>
where
  K: Clone,
  H: Clone,
{
  fn linear(&self) -> (Vec<Option<H>>, Vec<Constrain<K, H>>);
}

pub trait ParallelHelper<K, H: Helper>
where
  K: Clone,
  H: Clone,
{
  fn parallel(&self) -> (Vec<Option<H>>, Vec<Constrain<K, H>>);
}

pub trait Helper {}

pub trait Solver<K, H: Helper>
where
  K: Clone,
  H: Clone,
{
  fn new(input: (impl InputHelper<K, H> + 'static)) -> Self;
  fn solve(&mut self) -> Vec<K>;
  fn timing(&self) -> Option<Duration>;
  fn counting(&self) -> usize;
}

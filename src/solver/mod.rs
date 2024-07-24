pub mod aes128;
pub mod helpers;

// Solver Traits
use std::time::Duration;
use tree_jump::Constrain;

pub trait InputHelper<K, I, H: Helper>
where
  K: Clone,
  I: Clone,
  H: Clone,
{
  fn phi(candidate: K, helper: &Option<H>) -> bool
  where
    Self: Sized;

  fn finalise(&self, unpermuted: &Vec<K>) -> Vec<K>
  where
    Self: Sized;

  fn candidates(&self, mask: Option<Vec<usize>>) -> Vec<Vec<K>>;
  fn conditions(&self) -> (Vec<Option<H>>, Vec<Constrain<I, H>>);
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
  fn par_dimensions(&self) -> (Vec<Vec<usize>>, Vec<usize>);
  fn par_skips(&self) -> (Vec<Vec<usize>>, Vec<usize>);
}

pub trait Helper {
  fn fix_candidate(&self, partial: &Vec<usize>) -> Vec<usize>;
}

pub trait Solver<K, I, H: Helper>
where
  K: Clone,
  I: Clone,
  H: Clone,
{
  fn new(input: (impl InputHelper<K, I, H> + 'static)) -> Self;
  fn solve(&mut self) -> Vec<K>;
  fn timing(&self) -> Option<Duration>;
  fn counting(&self) -> usize;
}

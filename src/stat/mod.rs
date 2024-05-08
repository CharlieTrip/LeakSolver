use crate::cipher::aes::AES;
use crate::generator::aes_generator::AESGenerator2Rounds as AESGen;
use crate::generator::Generator;
use crate::leakfun::hw8::Hamming8;
use crate::leakfun::LeakFun;
use crate::solver::aes_solver::AESSolver;

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;

const HWSCORE: [u8; 9] = [0, 1, 2, 3, 4, 3, 2, 1, 0];
const BINOM: [u8; 9] = [1, 8, 28, 56, 70, 56, 28, 8, 1];

pub fn random_test(tot: usize, weight: u8, inv: bool) {
  let mut rng = rand::thread_rng();

  let sty = ProgressStyle::with_template(
            "{msg} {spinner:.green} [{elapsed_precise}] [{bar:25.cyan/blue}] ({pos}/{len}, {eta} ({per_sec})",
        )
        .unwrap()
        .progress_chars("##-");

  let pb = ProgressBar::new(tot as u64);
  pb.set_style(sty.clone());
  pb.set_message(format!("Test {} {}", weight, inv));

  for _ in 0..tot {
    pb.inc(1);
    let h8 = Hamming8 {};

    let x: Vec<u8> = (0..16).map(|_| rng.gen()).collect();

    // Generate an array of 16 uniformly random u8 elements
    let mut k = rnd_vec(weight, inv);

    // Fix Key to preserve the weight out desired
    for i in 0..16 {
      k[i] ^= x[i];
    }

    let lf = h8.get_leak_f();
    let gen = AESGen::generate(&x, &k, lf);

    // Print the generated array

    let mut w = 0;
    let mut win = 0;
    let mut wout = 0;
    let mut wfin = 0;
    let lf = h8.get_leak_f();

    for i in 0..16 {
      let j = lf(&k[i]) as usize;
      w += gen[i];
      win += HWSCORE[j];
      wout += HWSCORE[gen[i] as usize];
      wfin += HWSCORE[gen[16 + i] as usize];
    }

    let mut solver = AESSolver::<u8>::new(
      &([x.clone()].to_vec()),
      &([gen].to_vec()),
      Box::new(h8) as Box<dyn LeakFun<u8, u8>>,
    );

    let (sols, dur) = solver.solve();

    println!(
      "{:?};{:?};{:?};{:?};{:?};{:?};{:?};{:?};{:?};{:?}",
      weight,
      w,
      win,
      wout,
      wfin,
      dur,
      sols.len(),
      sols.contains(&k),
      x,
      k
    );
  }
  pb.finish_and_clear();
}

/// Return a vector with a given HW
pub fn rnd_vec(weight: u8, inv: bool) -> Vec<u8> {
  let mut rng = rand::thread_rng();
  let mut w: Vec<u8> = [0; 16].to_vec();

  for _ in 0..weight {
    let i = rng.gen::<usize>() % 16;
    w[i] = (w[i] + 1) % 9;
  }

  for i in 0..16 {
    let j = rng.gen::<usize>() % (BINOM[w[i] as usize] as usize);
    w[i] = match inv {
      false => AES::si(Hamming8::inv_leak_f(w[i])[j]),
      true => AES::si(Hamming8::inv_leak_f(8 - w[i])[j]),
    };
  }

  w
}

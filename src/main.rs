#![allow(dead_code)]

mod cipher;
mod generator;
mod leakfun;
mod solver;
mod stat;

use crate::generator::aes_generator::AESGenerator2Rounds as AESGen;
use crate::generator::Generator;
use crate::leakfun::hw8::Hamming8;
use crate::leakfun::LeakFun;
use crate::solver::aes_solver::AESSolver;
use crate::stat::random_test;

use std::env;

fn main() {
  // min_test();

  let args: Vec<String> = env::args().collect();

  // Check if both arguments are provided
  if args.len() < 3 {
    println!("Usage: ./leak_solver num_tests weight");
    return;
  }

  let tests: usize = match args[1].parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Error: num_tests argument is not a valid number.");
      return;
    }
  };

  let spec: u8 = match args[2].parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Error: weight argument is not a valid number.");
      return;
    }
  };

  for i in spec..(spec + 1) {
    random_test(tests, i, false);
    random_test(tests, i, true);
  }
}

fn min_test() {
  let i: u8 = 0;
  let x: Vec<u8> = [i; 16].to_vec();
  let k: Vec<u8> = [82, 82, 82, 0, 0, 82, 0, 0, 0, 0, 82, 0, 0, 82, 82, 82].to_vec();

  let h8 = Hamming8 {};
  let lf = h8.get_leak_f();

  let gen = AESGen::generate(&x, &k, lf);

  println!("x: {:?}\nk: {:?}\nl: {:?}", x, k, gen);

  let mut solver = AESSolver::<u8>::new(
    &([x].to_vec()),
    &([gen].to_vec()),
    Box::new(h8) as Box<dyn LeakFun<u8, u8>>,
  );

  let (sols, dur) = solver.solve();

  println!(
    "time: {:?}\nsols: {:?} <- {:?}",
    dur,
    sols.len(),
    sols.contains(&k)
  );
}

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
use crate::solver::aes_solver_jump::AESSolverJump;
use crate::stat::random_test_jump;
use leak_solver::solver::aes_solver::AESSolverLinear;
use leak_solver::solver::aes_solver_old::AESSolver;
use leak_solver::solver::helpers::aes_lin_helper::AESInputHelper;
use leak_solver::solver::Solver;
use std::time::Duration;

use std::env;

fn main() {
  test();
}

fn prompt() {
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
    random_test_jump(tests, i, false);
    random_test_jump(tests, i, true);
  }
}

fn min_test() {
  let i: u8 = 0;
  let x: Vec<u8> = [i; 16].to_vec();
  let k: Vec<u8> = [82, 82, 82, 0, 0, 82, 0, 0, 0, 0, 82, 0, 0, 82, 82, 82].to_vec();

  let lf = Hamming8::leak_f;
  let ilf = Hamming8::inv_leak_f;

  let gen = AESGen::generate(&x, &k, lf);

  println!("x: {:?}\nk: {:?}\nl: {:?}", x, k, gen);

  let mut solver = AESSolver::new(&([x].to_vec()), &([gen].to_vec()), (lf, ilf));

  let sols = solver.solve();
  let dur = solver.timing();
  let dur = match dur {
    Some(x) => x,
    None => Duration::new(0, 0),
  };

  println!(
    "time: {:?}\nsols: {:?} <- {:?}",
    dur,
    sols.len(),
    sols.contains(&k)
  );
}

fn test() {
  let i: u8 = 0;
  let x: Vec<u8> = [i; 16].to_vec();
  let k: Vec<u8> = [82, 82, 82, 0, 0, 82, 0, 0, 0, 0, 82, 0, 0, 82, 82, 82].to_vec();
  //let k: Vec<u8> = [82, 82, 82, 82, 82, 82, 82, 82, 0, 82, 9, 0, 0, 82, 82, 82].to_vec();

  let lf = Hamming8::leak_f;
  let ilf = Hamming8::inv_leak_f;

  let gen = AESGen::generate(&x, &k, lf);

  let inputs = AESInputHelper::new([x.clone()].to_vec(), [gen.clone()].to_vec(), (lf, ilf));
  let mut solver = AESSolverLinear::new(inputs);

  let sols = solver.solve();
  let dur = solver.timing();

  print!(
    "time: {:?} sols: {:?} <- {:?} , {:?}\n",
    dur,
    sols.len(),
    sols.contains(&k),
    solver.counting(),
  );

  let mut solver = AESSolverJump::new(&([x].to_vec()), &([gen].to_vec()), (lf, ilf));

  let sols = solver.solve();
  let dur = solver.timing();

  print!(
    "time: {:?} sols: {:?} <- {:?} , {:?}\n",
    dur,
    sols.len(),
    sols.contains(&k),
    solver.counting(),
  );
}

mod cipher;
mod generator;
mod leakfun;
mod solver;
mod stat;

use crate::generator::aes_generator::AESGenerator2Rounds as AESGen;
use crate::generator::Generator;
use crate::leakfun::hw8::Hamming8;
use crate::leakfun::LeakFun;

use crate::solver::aes128::aes128_solver::AESSolverLinear;
use crate::solver::aes128::aes128_solver_jump::AESSolverJump;
use crate::solver::aes128::aes128_solver_old::AESSolver;
use crate::solver::aes128::aes128_solver_par::AESSolverParallel;

use crate::solver::helpers::aes128_helper_lin::AESInputHelper as AESLin;
use crate::solver::helpers::aes128_helper_par::AESInputHelper as AESPar;
use crate::solver::Solver;

use crate::stat::random_test_jump;

use std::env;

fn main() {
  test();
}

fn test() {
  let i: u8 = 0;
  let x: Vec<u8> = [i; 16].to_vec();
  let _k: Vec<u8> = [82, 82, 82, 0, 0, 82, 0, 0, 0, 0, 82, 0, 0, 82, 82, 82].to_vec();
  let k: Vec<u8> = [82, 82, 82, 9, 9, 82, 9, 9, 9, 9, 82, 0, 9, 82, 82, 82].to_vec();

  let lf = Hamming8::leak_f;
  let ilf = Hamming8::inv_leak_f;

  let gen = AESGen::generate(&x, &k, lf);

  let mut solver = AESSolver::new(
    &([x.clone()].to_vec()),
    &([gen.clone()].to_vec()),
    (lf, ilf),
  );

  let sols = solver.solve();
  let dur = solver.timing();

  print!(
    "Oldest : {:?} sols: {:?} <- {:?} , {:?}\n",
    dur,
    sols.len(),
    sols.contains(&k),
    solver.counting(),
  );

  let mut solver = AESSolverJump::new(
    &([x.clone()].to_vec()),
    &([gen.clone()].to_vec()),
    (lf, ilf),
  );

  let sols = solver.solve();
  let dur = solver.timing();

  print!(
    "Old Lin: {:?} sols: {:?} <- {:?} , {:?}\n",
    dur,
    sols.len(),
    sols.contains(&k),
    solver.counting(),
  );

  let inputs = AESLin::new([x.clone()].to_vec(), [gen.clone()].to_vec(), (lf, ilf));
  let mut solver = AESSolverLinear::new(inputs);

  let sols = solver.solve();
  let dur = solver.timing();

  print!(
    "New Lin: {:?} sols: {:?} <- {:?} , {:?}\n",
    dur,
    sols.len(),
    sols.contains(&k),
    solver.counting(),
  );

  let inputs = AESPar::new([x.clone()].to_vec(), [gen.clone()].to_vec(), (lf, ilf));
  let mut solver = AESSolverParallel::new(inputs);

  let sols = solver.solve();
  let dur = solver.timing();

  print!(
    "New Par: {:?} sols: {:?} <- {:?} , {:?}\n",
    dur,
    sols.len(),
    sols.contains(&k),
    solver.counting(),
  );
}

fn _prompt() {
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

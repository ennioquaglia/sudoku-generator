#![allow(unused_imports)]
use sudokulib::{generator::generate_fully_solved_sudoku, grid::SudokuGrid, solver::solve_sudoku};
extern crate rand;
extern crate rand_chacha;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    let s = generate_fully_solved_sudoku(42);

    println!("{}", s);

    assert!(s.is_complete_and_correct());
}

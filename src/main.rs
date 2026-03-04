#![allow(unused_imports)]
use itertools::Itertools;
use sudokulib::{
    generator::generate_sudoku,
    grid::{SudokuGrid, to_sudoku_coord},
    solver::solve_sudoku,
};
extern crate rand;
extern crate rand_chacha;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn generate_sudoku_exchange_paste_code(s: &SudokuGrid) -> String {
    s.data.iter().join("")
}

fn main() {
    let mut rng = rand::rng();
    let sudoku_seed = rng.random::<u64>();
    let stencil_seed = rng.random::<u64>();

    println!(
        "sudoku_seed: {}, stencil_seed: {}",
        sudoku_seed, stencil_seed
    );

    let s = generate_sudoku(sudoku_seed, stencil_seed);

    println!("{}", s);
    println!(
        "https://sudokuexchange.com/play/?s={}\n",
        generate_sudoku_exchange_paste_code(&s)
    );

    assert!(s.has_unique_solution());
    assert!(s.is_incomplete());
}

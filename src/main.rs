#![allow(unused_imports)]
use sudokulib::{
    generator::{generate_fully_solved_sudoku, generate_sudoku_applying_minimal_stencil},
    grid::{SudokuGrid, to_sudoku_coord},
    solver::solve_sudoku,
};
extern crate rand;
extern crate rand_chacha;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    let s = generate_fully_solved_sudoku(42);

    println!("{}", s);

    assert!(s.is_complete_and_correct());

    let s = generate_sudoku_applying_minimal_stencil(s, 42);

    println!("{}", s);

    assert!(s.has_unique_solution());
    assert!(s.is_incomplete());
}

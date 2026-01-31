use rand_chacha::ChaCha8Rng;
extern crate rand;
extern crate rand_chacha;
use crate::{grid::SudokuGrid, solver::solve_sudoku_with_rng};
use rand::prelude::*;

pub fn generate_fully_solved_sudoku(seed: u64) -> SudokuGrid {
    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);

    let starter_sudoku = SudokuGrid::default();

    solve_sudoku_with_rng(starter_sudoku, &mut rng).unwrap()
}

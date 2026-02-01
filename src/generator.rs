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

// The sudoku is generated removing some values, or 'covering' them like when using a stencil/mask.
// The maximum number of values are removed following the order dictated by the seed, while keeping the solution unique.
pub fn generate_sudoku_applying_minimal_stencil(
    full_sudoku: SudokuGrid,
    stencil_seed: u64,
) -> SudokuGrid {
    assert!(full_sudoku.is_complete_and_correct());

    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(stencil_seed);

    let mut stencil_order: Vec<usize> = (0..81).collect();
    stencil_order.shuffle(&mut rng);

    let stencil_size_search_space: Vec<usize> = (0..81).collect();

    let mut sudoku = full_sudoku;

    _ = stencil_size_search_space.partition_point(|stencil_size| {
        sudoku = full_sudoku;

        for cell in stencil_order.iter().take(*stencil_size) {
            sudoku.data[*cell] = 0;
        }

        sudoku.has_unique_solution()
    });

    sudoku
}

pub fn generate_sudoku(sudoku_seed: u64, stencil_seed: u64) -> SudokuGrid {
    let full_sudoku = generate_fully_solved_sudoku(sudoku_seed);
    generate_sudoku_applying_minimal_stencil(full_sudoku, stencil_seed)
}

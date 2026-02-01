use std::collections::HashSet;
extern crate rand;
extern crate rand_chacha;
use rand::seq::SliceRandom;

use itertools::Itertools;
use rand_chacha::ChaCha8Rng;

use crate::{
    check::SudokuError,
    grid::{SudokuGrid, to_sudoku_coord, to_sudoku_subrect_index},
};

#[derive(Debug, Default)]
pub enum SolverType<'a> {
    #[default]
    SequentialFirst,
    Sequential,
    CheckUnique,
    Rng(&'a mut ChaCha8Rng),
}

#[derive(Debug, Default)]
pub enum SolverError {
    #[default]
    SolutionNotFound,
    SolutionNotUnique,
    SudokuError(SudokuError),
}

pub fn solve_sudoku(sudoku: SudokuGrid) -> Result<SudokuGrid, SudokuError> {
    solve_sudoku_helper(sudoku, &mut SolverType::Sequential)
        .map(|results| {
            assert!(results.len() == 1);
            results[0]
        })
        .map_err(|err| match err {
            SolverError::SudokuError(e) => e,
            _ => SudokuError::default(),
        })
}

pub fn solve_sudoku_with_rng(
    sudoku: SudokuGrid,
    rng: &mut ChaCha8Rng,
) -> Result<SudokuGrid, SudokuError> {
    solve_sudoku_helper(sudoku, &mut SolverType::Rng(rng))
        .map(|results| {
            assert!(results.len() == 1);
            results[0]
        })
        .map_err(|err| match err {
            SolverError::SudokuError(e) => e,
            _ => SudokuError::default(),
        })
}

pub fn solve_sudoku_helper(
    sudoku: SudokuGrid,
    solver: &mut SolverType,
) -> Result<Vec<SudokuGrid>, SolverError> {
    let mut results: HashSet<SudokuGrid> = HashSet::new();
    let mut sudoku = sudoku;

    while sudoku.is_incomplete() && sudoku.check_correct(true).is_ok() {
        let empty_cells_indicies: Vec<usize> = sudoku
            .data
            .into_iter()
            .enumerate()
            .filter(|(_i, v)| *v == 0)
            .map(|(i, _v)| i)
            .collect();

        let mut possible_moves: Vec<(usize, Vec<u8>)> = Vec::new();

        for cell in empty_cells_indicies {
            let (x, y) = to_sudoku_coord(cell);
            let mut values: HashSet<u8> = HashSet::new();

            values.extend(sudoku.row(y));
            values.extend(sudoku.column(x));
            values.extend(sudoku.rect(to_sudoku_subrect_index(cell)));

            let possible_values: Vec<u8> = (1..=9).filter(|v| !values.contains(v)).collect();

            if possible_values.is_empty() {
                return Err(SolverError::SolutionNotFound);
            }

            possible_moves.push((cell, possible_values));
        }

        possible_moves.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        if let Some((number_of_possible_values, group)) =
            (&possible_moves.into_iter().chunk_by(|v| v.1.len()))
                .into_iter()
                .next()
        {
            assert!(number_of_possible_values > 0);

            if number_of_possible_values == 1 {
                for (cell, possible_values) in group {
                    sudoku.data[cell] = possible_values[0];
                }
            } else {
                let mut group: Vec<_> = group.collect();

                if let SolverType::Rng(rng) = solver {
                    group.shuffle(rng);
                }

                for (cell, mut possible_values) in group {
                    if let SolverType::Rng(rng) = solver {
                        possible_values.shuffle(rng);
                    }

                    for v in possible_values {
                        let mut new_sudoku = sudoku;
                        new_sudoku.data[cell] = v;

                        let result = solve_sudoku_helper(new_sudoku, solver);
                        if let Ok(solutions) = result {
                            results.extend(solutions);

                            match solver {
                                SolverType::CheckUnique => {
                                    if results.len() > 1 {
                                        return Err(SolverError::SolutionNotUnique);
                                    }
                                }
                                SolverType::Sequential => {}
                                _ => {
                                    assert!(results.len() == 1);
                                    return Ok(results.into_iter().collect_vec());
                                }
                            }
                        } else if let Err(SolverError::SolutionNotUnique) = result {
                            return result;
                        }
                    }
                    // continue;
                }

                if let SolverType::Sequential = solver
                    && !results.is_empty()
                {
                    return Ok(results.into_iter().collect_vec());
                }

                return Err(SolverError::SolutionNotFound);
            }
        }
    }

    let result = sudoku.check_correct(false);
    let mut error = SolverError::default();

    match result {
        Ok(_) => {
            results.insert(sudoku);
        }
        Err(err) => {
            error = SolverError::SudokuError(err);
        }
    }

    match (solver, results.len()) {
        (SolverType::CheckUnique, 2..) => Err(SolverError::SolutionNotUnique),
        (_, 0) => Err(error),
        _ => Ok(results.into_iter().collect_vec()),
    }
}

#[test]
fn solver_test() {
    let original = SudokuGrid::fill_random();
    let mut s = original;

    for i in [2, 6] {
        s.rect_mut(i).for_each(|v| *v = 0);
    }

    assert!(!s.is_complete_and_correct());
    assert!(s.is_incomplete());
    assert!(s.check_correct(true).is_ok());

    println!("{}", s);

    let r = solve_sudoku(s);

    match r {
        Ok(s) => {
            println!("solution: \n{}", s);

            assert!(s.is_complete_and_correct());
            assert!(!s.is_incomplete());
            assert!(s.check_correct(true).is_ok());
            assert_eq!(original, s);
        }
        Err(err) => panic!("cannot find a solution : {:?}", err),
    }
}

#[test]
fn solver_sudoku_without_two_corner_rect_has_unique_solution() {
    let original = SudokuGrid::fill_random();
    let mut s = original;

    for i in [2, 6] {
        s.rect_mut(i).for_each(|v| *v = 0);
    }

    assert!(!s.is_complete_and_correct());
    assert!(s.is_incomplete());
    assert!(s.check_correct(true).is_ok());

    println!("{}", s);

    assert!(s.has_unique_solution());
}

#[test]
fn solver_sudoku_without_four_rect_has_multiple_solutions() {
    let original = SudokuGrid::fill_random();
    let mut s = original;

    for i in [2, 3, 5, 6] {
        s.rect_mut(i).for_each(|v| *v = 0);
    }

    assert!(!s.is_complete_and_correct());
    assert!(s.is_incomplete());
    assert!(s.check_correct(true).is_ok());

    println!("{}", s);

    assert!(!s.has_unique_solution());
}

#[test]
fn solver_sudoku_without_four_rect_get_multiple_solutions() {
    let original = SudokuGrid::fill_random();
    let mut s = original;

    for i in [2, 3, 6] {
        s.rect_mut(i).for_each(|v| *v = 0);
    }

    assert!(!s.is_complete_and_correct());
    assert!(s.is_incomplete());
    assert!(s.check_correct(true).is_ok());

    println!("{}", s);

    let result = solve_sudoku_helper(s, &mut SolverType::Sequential);
    assert!(!s.has_unique_solution());
    assert!(result.is_ok());

    match result {
        Ok(solutions) => {
            for (i, solution) in solutions.iter().enumerate() {
                println!("solution {}:\n{}", i + 1, solution);
            }
            assert!(solutions.len() > 1);
            assert!(solutions.iter().all_unique());
        }
        Err(err) => {
            panic!("error: {:?}", err);
        }
    };
}

#[test]
fn solver_sequential_find_one_solution() {
    let original = SudokuGrid::fill_random();
    let mut s = original;

    for i in [2, 6] {
        s.rect_mut(i).for_each(|v| *v = 0);
    }

    assert!(!s.is_complete_and_correct());
    assert!(s.is_incomplete());
    assert!(s.check_correct(true).is_ok());

    println!("{}", s);

    let result = solve_sudoku_helper(s, &mut SolverType::Sequential);

    assert!(result.is_ok());

    let solutions = result.unwrap();
    assert!(solutions.len() == 1);
}

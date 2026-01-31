use std::collections::HashSet;

use crate::{
    check::SudokuError,
    grid::{SudokuGrid, to_sudoku_coord, to_sudoku_subrect_index},
};

pub fn solve_sudoku(sudoku: SudokuGrid) -> Result<SudokuGrid, SudokuError> {
    let mut sudoku = sudoku;

    while sudoku.is_incomplete() {
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

            let possible_values: Vec<u8> = (1..10).filter(|v| !values.contains(v)).collect();

            if possible_values.is_empty() {
                return Err(SudokuError::default());
            }

            possible_moves.push((cell, possible_values));
        }

        possible_moves.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        for (cell, possible_values) in possible_moves {
            if possible_values.len() == 1 {
                sudoku.data[cell] = possible_values[0];
            } else {
                for v in possible_values {
                    // try all moves
                    sudoku.data[cell] = v;
                    let result = solve_sudoku(sudoku);
                    if result.is_ok() {
                        return result;
                    }
                }
                return Err(SudokuError::default());
            }
        }
    }

    sudoku.check_correct(false).map(|_| sudoku)
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

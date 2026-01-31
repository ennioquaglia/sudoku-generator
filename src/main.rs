use sudokulib::{grid::SudokuGrid, solver::solve_sudoku};

fn main() {
    // let s = SudokuGrid::default();
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
        Err(err) => println!("cannot find a solution : {:?}", err),
    }
}

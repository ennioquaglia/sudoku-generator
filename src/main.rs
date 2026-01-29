use sudokulib::grid::SudokuGrid;


fn main() {
    // let s = SudokuGrid::default();
    let s = SudokuGrid::fill_random();
    assert!(s.check_correct().is_ok());
    
    println!("{}", s);
}

use sudokulib::grid::SudokuGrid;


fn main() {
    // let s = SudokuGrid::default();
    let s = SudokuGrid::fill_random();

    println!("{}", s);
}

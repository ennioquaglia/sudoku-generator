use crate::{
    grid::SudokuGrid,
    solver::{SolverType, solve_sudoku_helper},
};

#[derive(Debug, Default)]
pub struct SudokuError {
    pub error_type: SudokuErrorType,
    pub cells_indicies: Vec<usize>,
    pub location: SudokuErrorLocation,
}
#[derive(Debug, Default)]
pub enum SudokuErrorType {
    #[default]
    Unknown,
    ValueRepeated,
    MissingValue,
    ValueOutOfRange,
}
#[derive(Debug, Default)]
pub enum SudokuErrorLocation {
    #[default]
    None,
    Row(usize),
    Column(usize),
    Rect(usize),
}

impl SudokuError {
    pub fn new(error_type: SudokuErrorType) -> Self {
        Self {
            error_type,
            ..Default::default()
        }
    }
    pub fn with_location(self, loc: SudokuErrorLocation) -> Self {
        Self {
            location: loc,
            ..self
        }
    }
    pub fn with_cells(self, indicies: Vec<usize>) -> Self {
        let mut new_indicies = self.cells_indicies;
        new_indicies.extend(indicies);
        Self {
            cells_indicies: new_indicies,
            ..self
        }
    }
}

impl SudokuGrid {
    fn check_group<I, T>(iter: I, allow_incomplete: bool) -> Result<(), SudokuError>
    where
        I: Iterator<Item = T>,
        T: Into<usize>,
    {
        let mut ok = [false; 9];
        for val in iter {
            let val = val.into();

            // no value
            if val == 0 {
                if allow_incomplete {
                    continue;
                } else {
                    return Err(SudokuError::new(SudokuErrorType::MissingValue));
                }
            }

            let i = val - 1;

            // value out of range
            if !(0..9).contains(&i) {
                return Err(SudokuError::new(SudokuErrorType::ValueOutOfRange));
            }

            // value repeated
            if ok[i] {
                return Err(SudokuError::new(SudokuErrorType::ValueRepeated));
            }

            ok[i] = true;
        }
        if allow_incomplete || ok.iter().all(|val| *val) {
            Ok(())
        } else {
            Err(SudokuError::new(SudokuErrorType::MissingValue))
        }
    }

    pub fn check_correct(&self, allow_incomplete: bool) -> Result<(), SudokuError> {
        for (i, row) in self.rows().enumerate() {
            Self::check_group(row, allow_incomplete)
                .map_err(|e| e.with_location(SudokuErrorLocation::Row(i)))?;
        }
        for (i, row) in self.columns().enumerate() {
            Self::check_group(row, allow_incomplete)
                .map_err(|e| e.with_location(SudokuErrorLocation::Column(i)))?;
        }
        for (i, row) in self.rects().enumerate() {
            Self::check_group(row, allow_incomplete)
                .map_err(|e| e.with_location(SudokuErrorLocation::Rect(i)))?;
        }
        Ok(())
    }

    pub fn is_incomplete(&self) -> bool {
        self.data.contains(&0)
    }

    pub fn is_complete_and_correct(&self) -> bool {
        self.check_correct(false).is_ok()
    }

    pub fn has_unique_solution(&self) -> bool {
        assert!(self.check_correct(true).is_ok());

        solve_sudoku_helper(*self, &mut SolverType::CheckUnique).is_ok()
    }
}

#[test]
fn test_sudoku_check() {
    let mut s = SudokuGrid::fill_random();

    assert!(s.is_complete_and_correct());
    assert!(!s.is_incomplete());

    let old = s.data[42];
    s.data[42] = 0;

    assert!(!s.is_complete_and_correct());
    assert!(s.is_incomplete());

    s.data[42] = (old - 2) % 9 + 1;

    assert!(!s.is_complete_and_correct());
    assert!(!s.is_incomplete());
}

#[test]
fn check_complete_sudoku_has_unique_solution() {
    let s = SudokuGrid::fill_random();
    assert!(!s.is_incomplete());
    assert!(s.has_unique_solution())
}

#[test]
fn check_complete_sudoku_but_one_cell_has_unique_solution() {
    let mut s = SudokuGrid::fill_random();
    s.data[42] = 0;
    assert!(s.is_incomplete());
    assert!(s.has_unique_solution())
}

#[test]
fn check_complete_sudoku_but_one_rect_has_unique_solution() {
    let mut s = SudokuGrid::fill_random();
    for cell in s.rect_mut(5) {
        *cell = 0;
    }
    assert!(s.is_incomplete());
    assert!(s.has_unique_solution())
}

use crate::grid::SudokuGrid;

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
            error_type: error_type,
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
    fn check_group<I, T>(iter: I) -> Result<(), SudokuError>
    where
        I: Iterator<Item = T>,
        T: Into<usize>,
    {
        let mut ok = [false; 9];
        for val in iter {
            let val = val.into();

            // no value
            if val == 0 {
                return Err(SudokuError::new(SudokuErrorType::MissingValue));
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
        if ok.iter().all(|val| *val) {
            Ok(())
        } else {
            Err(SudokuError::new(SudokuErrorType::MissingValue))
        }
    }

    pub fn check_correct(&self) -> Result<(), SudokuError> {
        for (i, row) in self.rows().enumerate() {
            Self::check_group(row).map_err(|e| e.with_location(SudokuErrorLocation::Row(i)))?;
        }
        for (i, row) in self.columns().enumerate() {
            Self::check_group(row).map_err(|e| e.with_location(SudokuErrorLocation::Column(i)))?;
        }
        for (i, row) in self.rects().enumerate() {
            Self::check_group(row).map_err(|e| e.with_location(SudokuErrorLocation::Rect(i)))?;
        }
        Ok(())
    }

    pub fn is_incomplete(&self) -> bool {
        self.data.iter().any(|v| *v == 0)
    }
}

#[test]
fn test_sudoku_check() {
    let mut s = SudokuGrid::fill_random();

    assert!(s.check_correct().is_ok());
    assert!(!s.is_incomplete());

    let old = s.data[42];
    s.data[42] = 0;

    assert!(s.check_correct().is_err());
    assert!(s.is_incomplete());

    s.data[42] = (old - 2) % 9 + 1;

    assert!(s.check_correct().is_err());
    assert!(!s.is_incomplete());
}

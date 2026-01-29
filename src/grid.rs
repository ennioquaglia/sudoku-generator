use std::fmt;

use itertools::Itertools;

pub fn to_sudoku_coord(i: usize) -> (usize, usize) {
    (i / 9, i % 9)
}

pub fn from_sudoku_coord(y: usize, x: usize) -> usize {
    y * 9 + x
}

#[derive(Copy, Clone)]
pub struct SudokuGrid {
    pub data: [u8; 9 * 9],
}

pub struct GridSliceIterator<'a> {
    indicies: Vec<usize>,
    grid: &'a SudokuGrid,
}
impl<'a> Iterator for GridSliceIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.indicies.pop() {
            let value = self.grid.data[index];
            return Some(value);
        }
        None
    }
}
impl<'a> fmt::Debug for GridSliceIterator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("GridSliceIterator")
            .field("indicies", &self.indicies)
            .finish()
    }
}

impl Default for SudokuGrid {
    fn default() -> Self {
        Self { data: [0; _] }
    }
}
impl SudokuGrid {
    pub fn fill_random() -> Self {
        let mut s = Self { data: [0; _] };
        for i in 0..81 {
            s.data[i] = ((i + (i / 9) * 3 + (i / 27)) % 9 + 1) as u8
        }
        s
    }
}

impl SudokuGrid {
    pub fn row(&self, y: usize) -> GridSliceIterator<'_> {
        GridSliceIterator {
            grid: self,
            indicies: (0..9).map(|i| i + y * 9).rev().collect(),
        }
    }
    pub fn rows(&self) -> impl Iterator<Item = GridSliceIterator<'_>> {
        (0..9).map(|i| self.row(i))
    }

    pub fn column(&self, x: usize) -> GridSliceIterator<'_> {
        GridSliceIterator {
            grid: self,
            indicies: (0..9).map(|i| i * 9 + x).rev().collect(),
        }
    }
    pub fn columns(&self) -> impl Iterator<Item = GridSliceIterator<'_>> {
        (0..9).map(|i| self.column(i))
    }
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h_line = |f: &mut fmt::Formatter<'_>,
                      left_corner: &str,
                      hline: &str,
                      intersection: &str,
                      intersection_heavy: &str,
                      right_corner: &str| {
            let h = hline.repeat(3);
            let s = std::iter::repeat_n(std::iter::repeat_n(h, 3).join(intersection), 3)
                .join(intersection_heavy);
            writeln!(f, "{}{}{}", left_corner, s, right_corner)
        };

        for (i, row) in self.rows().enumerate() {
            match i {
                0 => h_line(f, "┏", "━", "┯", "┳", "┓")?,
                3 | 6 => h_line(f, "┣", "━", "┿", "╋", "┫")?,
                _ => h_line(f, "┣", "─", "┼", "╂", "┨")?,
            };

            writeln!(
                f,
                "┃ {} ┃",
                row.into_iter()
                    .chunks(3)
                    .into_iter()
                    .map(|c| {
                        c.into_iter()
                            .map(|value| {
                                if value == 0 {
                                    " ".to_string()
                                } else {
                                    value.to_string()
                                }
                            })
                            .join(" │ ")
                    })
                    .join(" ┃ ")
            )?;
        }
        h_line(f, "┗", "━", "┻", "┻", "┛")?;
        Ok(())
    }
}

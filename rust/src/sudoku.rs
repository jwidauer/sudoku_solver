use anyhow::{bail, Result};
use ndarray::{iter, prelude::*};
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone)]
pub struct Sudoku {
    board: Array2<u8>,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {
            board: Array2::zeros((9, 9)),
        }
    }

    pub fn serialize(&self) -> String {
        self.board
            .iter()
            .map(|cell| match *cell {
                0 => '.',
                _ => (*cell + b'0') as char,
            })
            .collect()
    }

    pub fn try_from_str(s: &str) -> Result<Sudoku> {
        if s.len() != 81 {
            bail!("Invalid sudoku string length: expected 81, got {}", s.len());
        }
        let mut board = Array2::zeros((9, 9));
        s.chars().zip(board.iter_mut()).for_each(|(c, u)| {
            *u = match c.to_digit(10) {
                Some(d) => d as u8,
                None => 0,
            };
        });
        Ok(Sudoku { board })
    }

    pub fn iter(&self) -> iter::Iter<'_, u8, Dim<[usize; 2]>> {
        self.board.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = ArrayView1<'_, u8>> {
        self.board.rows().into_iter()
    }

    pub fn cols(&self) -> impl Iterator<Item = ArrayView1<'_, u8>> {
        self.board.columns().into_iter()
    }

    pub fn boxes(&self) -> impl Iterator<Item = Array1<&u8>> {
        self.board
            .exact_chunks((3, 3))
            .into_iter()
            .map(|block| block.into_iter().collect::<Array<_, _>>())
    }

    pub fn row(&self, idx: usize) -> ArrayView1<'_, u8> {
        self.board.row(idx)
    }

    pub fn col(&self, idx: usize) -> ArrayView1<'_, u8> {
        self.board.column(idx)
    }

    pub fn box_containing(&self, row: usize, col: usize) -> Array1<u8> {
        let start_row = 3 * (row / 3);
        let start_col = 3 * (col / 3);

        let end_row = start_row + 3;
        let end_col = start_col + 3;

        self.board
            .slice(s![start_row..end_row, start_col..end_col])
            .to_shape(9)
            .unwrap()
            .into_owned()
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.board[[row, col]]
    }

    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.board[[row, col]] = val
    }

    pub fn get_box_idx(&self, row: usize, col: usize) -> usize {
        (row / 3) * 3 + col / 3
    }

    pub fn is_valid(&self, row: usize, col: usize, val: u8) -> bool {
        // Check that val is within [1, 9]
        if !(1..=9).contains(&val) {
            return false;
        }

        let is_val = |elem: &u8| *elem == val;

        // Check row constraint
        if self.row(row).iter().any(is_val) {
            return false;
        }

        // Check col constraint
        if self.col(col).iter().any(is_val) {
            return false;
        }

        // Check box constraint
        if self.box_containing(row, col).iter().any(is_val) {
            return false;
        }

        true
    }

    pub fn is_solved(&self) -> bool {
        // Check that all cells are in range [1, 9]
        if self.board.iter().any(|cell| !(1..=9).contains(cell)) {
            return false;
        }

        // Check row constraints
        if self
            .rows()
            .any(|row| row.iter().collect::<HashSet<_>>().len() != 9)
        {
            return false;
        }

        // Check column constraints
        if self
            .cols()
            .any(|col| col.iter().collect::<HashSet<_>>().len() != 9)
        {
            return false;
        }

        // Check box constraints
        if self
            .boxes()
            .any(|box_| box_.iter().collect::<HashSet<_>>().len() != 9)
        {
            return false;
        }

        true
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Sudoku {
    type Item = &'a u8;
    type IntoIter = iter::Iter<'a, u8, Dim<[usize; 2]>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl TryFrom<&str> for Sudoku {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Sudoku> {
        Sudoku::try_from_str(s)
    }
}

impl TryFrom<[u8; 81]> for Sudoku {
    type Error = anyhow::Error;

    fn try_from(board: [u8; 81]) -> Result<Sudoku> {
        if board.iter().any(|elem| *elem > 9) {
            bail!("Invalid board");
        }
        Ok(Sudoku {
            board: Array2::from_shape_vec((9, 9), board.to_vec()).unwrap(),
        })
    }
}

impl From<Sudoku> for String {
    fn from(sudoku: Sudoku) -> String {
        sudoku.serialize()
    }
}

impl From<Sudoku> for [u8; 81] {
    fn from(sudoku: Sudoku) -> [u8; 81] {
        sudoku.board.into_raw_vec_and_offset().0.try_into().unwrap()
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, row) in self.rows().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match *cell {
                    0 => s.push('.'),
                    _ => s.push((*cell + b'0') as char),
                }

                if j == 2 || j == 5 {
                    s.push('|');
                }
            }
            s.push('\n');
            if i == 2 || i == 5 {
                s.push_str("---+---+---\n");
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() -> Result<()> {
        let sudoku = Sudoku::new();
        assert_eq!(
            sudoku.serialize(),
            "................................................................................."
        );
        assert!(!sudoku.is_solved());

        let sudoku: Sudoku =
            "................................................................................."
                .try_into()?;
        assert_eq!(
            sudoku.serialize(),
            "................................................................................."
        );
        assert!(!sudoku.is_solved());

        let sudoku: Sudoku =
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
                .try_into()?;
        assert_eq!(
            sudoku.serialize(),
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
        );
        assert!(sudoku.is_solved());

        let sudoku: Sudoku =
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567."
                .try_into()?;
        assert_eq!(
            sudoku.serialize(),
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567."
        );
        assert!(!sudoku.is_solved());

        let sudoku: Sudoku =
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567a"
                .try_into()?;
        assert_eq!(
            sudoku.serialize(),
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567."
        );
        assert!(!sudoku.is_solved());

        Ok(())
    }

    #[test]
    fn test_validity_check() -> Result<()> {
        let sudoku = Sudoku::new();
        assert!(!sudoku.is_valid(0, 0, 0));
        assert!(sudoku.is_valid(0, 0, 1));
        assert!(!sudoku.is_valid(0, 0, 10));

        let sudoku: Sudoku =
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
                .try_into()?;
        assert!(!sudoku.is_valid(0, 0, 0));
        assert!(!sudoku.is_valid(0, 0, 1));
        assert!(!sudoku.is_valid(0, 0, 10));

        let sudoku: Sudoku =
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567."
                .try_into()?;
        assert!(!sudoku.is_valid(0, 0, 0));
        assert!(!sudoku.is_valid(0, 0, 1));
        assert!(!sudoku.is_valid(0, 0, 10));
        assert!(sudoku.is_valid(8, 8, 8));

        Ok(())
    }

    #[test]
    fn test_get() -> Result<()> {
        let sudoku = Sudoku::new();
        assert_eq!(sudoku.get(0, 0), 0);
        assert_eq!(sudoku.get(8, 8), 0);

        let sudoku: Sudoku =
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
                .try_into()?;
        assert_eq!(sudoku.get(0, 0), 1);
        assert_eq!(sudoku.get(8, 8), 8);

        let sudoku: Sudoku =
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567."
                .try_into()?;
        assert_eq!(sudoku.get(0, 0), 1);
        assert_eq!(sudoku.get(8, 8), 0);

        Ok(())
    }

    #[test]
    fn test_set() -> Result<()> {
        let mut sudoku = Sudoku::new();
        assert_eq!(sudoku.get(0, 0), 0);
        assert_eq!(sudoku.get(8, 8), 0);
        sudoku.set(0, 0, 1);
        sudoku.set(8, 8, 9);
        assert_eq!(sudoku.get(0, 0), 1);
        assert_eq!(sudoku.get(8, 8), 9);

        let mut sudoku: Sudoku =
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
                .try_into()?;
        assert_eq!(sudoku.get(0, 0), 1);
        assert_eq!(sudoku.get(8, 8), 8);
        sudoku.set(0, 0, 2);
        sudoku.set(8, 8, 9);
        assert_eq!(sudoku.get(0, 0), 2);
        assert_eq!(sudoku.get(8, 8), 9);

        let mut sudoku: Sudoku =
            "12345678945678912378912345623456789156789123489123456734567891267891234591234567."
                .try_into()?;
        assert_eq!(sudoku.get(0, 0), 1);
        assert_eq!(sudoku.get(8, 8), 0);
        sudoku.set(0, 0, 2);
        sudoku.set(8, 8, 9);
        assert_eq!(sudoku.get(0, 0), 2);
        assert_eq!(sudoku.get(8, 8), 9);

        Ok(())
    }
}

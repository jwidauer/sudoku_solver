use super::sudoku::Sudoku;

pub trait SudokuSolver {
    fn solve(&self, board: Sudoku) -> Option<Sudoku>;
}

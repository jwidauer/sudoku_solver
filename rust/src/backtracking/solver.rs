use crate::solver::SudokuSolver;
use crate::sudoku::Sudoku;

pub struct Solver {}

impl Solver {
    pub fn new() -> Self {
        Self {}
    }

    fn solve_inplace(board: &mut Sudoku) -> bool {
        let Some(idx) = board.iter().position(|elem| *elem == 0) else {
            return true;
        };

        let row = idx / 9;
        let col = idx % 9;

        for i in 1..10 {
            if !board.is_valid(row, col, i) {
                continue;
            }

            board.set(row, col, i);
            if Self::solve_inplace(board) {
                return true;
            }
            board.set(row, col, 0);
        }
        false
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl SudokuSolver for Solver {
    fn solve(&self, mut board: Sudoku) -> Option<Sudoku> {
        Self::solve_inplace(&mut board).then_some(board)
    }
}

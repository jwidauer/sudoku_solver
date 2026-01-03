use crate::{solver::SudokuSolver, sudoku::Sudoku};

use super::algorithm::NodeGrid;

pub const NR_CANDIDATES: usize = 9 * 9 * 9; // 729
pub const NR_CONSTRAINTS: usize = 4 * 9 * 9; // 324

#[derive(Debug)]
struct Candidate {
    row: u8,
    col: u8,
    num: u8,
}

pub struct Solver {
    sparse_mat: Vec<[u16; 4]>,
    candidates: Vec<Candidate>,
}

impl Solver {
    pub fn new() -> Self {
        let candidates: Vec<_> = (1..10)
            .flat_map(|row| (1..10).flat_map(move |col| (1..10).map(move |num| (row, col, num))))
            .map(|(row, col, num)| Candidate { row, col, num })
            .collect();

        // Store just the column indices of the constraints for each candidate
        let mut sparse_mat: Vec<[u16; 4]> = Vec::with_capacity(NR_CANDIDATES);

        for candidate in &candidates {
            let row = (candidate.row - 1) as usize;
            let col = (candidate.col - 1) as usize;
            let num = (candidate.num - 1) as usize;

            // Cell constraint
            // Each cell (row, col) must be filled with exactly one number
            let cell_cons_col = row * 9 + col;

            // Row constraint
            // Each number must appear exactly once in each row
            let row_cons_col = row * 9 + num + 81;

            // Column constraint
            // Each number must appear exactly once in each column!
            let col_cons_col = col * 9 + num + 2 * 81;

            // Box constraint
            // Each number must appear exactly once in each 3x3 box
            let box_row = row / 3;
            let box_col = col / 3;
            let box_index = box_row * 3 + box_col;
            let box_cons_col = box_index * 9 + num + 3 * 81;

            sparse_mat.push([
                cell_cons_col as u16,
                row_cons_col as u16,
                col_cons_col as u16,
                box_cons_col as u16,
            ]);
        }

        Self {
            sparse_mat,
            candidates,
        }
    }

    fn calc_row_idcs(board: &Sudoku) -> Vec<usize> {
        let mut row_idcs = Vec::with_capacity(NR_CANDIDATES);
        for (i, &elem) in board.iter().enumerate() {
            let row = i / 9;
            let col = i % 9;

            if elem == 0 {
                // If the cell is empty, we need to consider all possible numbers (1-9)
                let idx = 81 * row + 9 * col;
                row_idcs.extend(idx..idx + 9);
            } else {
                // If the cell is filled, we only consider that specific number
                let num = (elem - 1) as usize;
                let idx = 81 * row + 9 * col + num;
                row_idcs.push(idx);
            }
        }
        row_idcs
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl SudokuSolver for Solver {
    fn solve(&self, mut board: Sudoku) -> Option<Sudoku> {
        // Prepare the list of row indices to select from the exact cover matrix
        let row_idcs = Self::calc_row_idcs(&board);

        // Create a sub-matrix containing only the relevant rows
        // let sub_matrix = self.matrix.select(Axis(0), &row_idcs);
        let sparse_sub_mat = row_idcs
            .iter()
            .map(|&idx| self.sparse_mat[idx])
            .collect::<Vec<_>>();
        let solution = NodeGrid::from_sparse_matrix(&sparse_sub_mat, NR_CONSTRAINTS).search()?;

        let candidates = row_idcs
            .into_iter()
            .map(|idx| &self.candidates[idx])
            .collect::<Vec<_>>();

        for idx in solution {
            let candidate = candidates[idx as usize];
            let row = (candidate.row - 1) as usize;
            let col = (candidate.col - 1) as usize;
            let num = candidate.num;

            board.set(row, col, num);
        }

        Some(board)
    }
}

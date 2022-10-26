use crate::{sudoku::Sudoku, sudoku_solver::SudokuSolver};

use super::algorithm_x::NodeGrid;
use ndarray::prelude::*;

const NR_CANDIDATES: usize = 9 * 9 * 9; // 729
const NR_CONSTRAINTS: usize = 4 * 9 * 9; // 324

#[derive(Debug)]
struct Candidate {
    row: u8,
    col: u8,
    num: u8,
}

pub struct AlgorithmXSudokuSolver {
    matrix: Array2<bool>,
    candidates: Vec<Candidate>,
}

impl AlgorithmXSudokuSolver {
    pub fn new() -> Self {
        let candidates: Vec<_> = (1..10)
            .flat_map(|row| (1..10).flat_map(move |col| (1..10).map(move |num| (row, col, num))))
            .map(|(row, col, num)| Candidate { row, col, num })
            .collect();

        let mut matrix = Array2::<bool>::default((NR_CANDIDATES, NR_CONSTRAINTS));
        for candidate in &candidates {
            let row = (candidate.row - 1) as usize;
            let col = (candidate.col - 1) as usize;
            let num = (candidate.num - 1) as usize;

            let matrix_row = (row * 81) + (col * 9) + num;

            // Cell constraint
            // Each cell (row, col) must be filled with exactly one number
            let cell_cons_col = row * 9 + col;
            matrix[[matrix_row, cell_cons_col]] = true;

            // Row constraint
            // Each number must appear exactly once in each row
            let row_cons_col = row * 9 + num + 81;
            matrix[[matrix_row, row_cons_col]] = true;

            // Column constraint
            // Each number must appear exactly once in each column!
            let col_cons_col = col * 9 + num + 2 * 81;
            matrix[[matrix_row, col_cons_col]] = true;

            // Box constraint
            // Each number must appear exactly once in each 3x3 box
            let box_row = row / 3;
            let box_col = col / 3;
            let box_index = box_row * 3 + box_col;
            let box_cons_col = box_index * 9 + num + 3 * 81;
            matrix[[matrix_row, box_cons_col]] = true;
        }

        Self { matrix, candidates }
    }
}

impl SudokuSolver for AlgorithmXSudokuSolver {
    fn solve(&self, mut board: Sudoku) -> Option<Sudoku> {
        // Prepare the list of row indices to select from the exact cover matrix
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

        let sub_matrix = self.matrix.select(Axis(0), &row_idcs);
        let solution = NodeGrid::new(sub_matrix).search()?;

        let candidates = row_idcs
            .into_iter()
            .map(|idx| &self.candidates[idx])
            .collect::<Vec<_>>();

        for idx in solution {
            let candidate = candidates[idx];
            let row = (candidate.row - 1) as usize;
            let col = (candidate.col - 1) as usize;
            let num = candidate.num;

            board.set(row, col, num);
        }

        Some(board)
    }
}

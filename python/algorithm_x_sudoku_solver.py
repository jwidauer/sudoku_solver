import numpy as np

from typing import Any, List
from algorithm_x import AlgorithmX


class Candidate:
    row: int
    col: int
    num: int

    def __init__(self, row, col, num):
        self.row = row
        self.col = col
        self.num = num

    def __str__(self):
        return "r" + str(self.row) + "c" + str(self.col) + "#" + str(self.num)


class AlgorithmXSudokuSolver:
    def __init__(self):
        # Set up candidate list
        candidates: List[Candidate] = []
        for row in range(1, 10):
            for col in range(1, 10):
                for num in range(1, 10):
                    candidates.append(Candidate(row, col, num))

        self.candidates = np.array(candidates, dtype=Candidate)

        # Set up constraints matrix
        # Add cell constraints
        tmp = np.zeros((9, 81))
        tmp[:, 0] = np.ones(9)
        cell_constraints = tmp
        for i in range(1, 81):
            cell_constraints = np.vstack((cell_constraints, np.roll(tmp, i)))

        self.matrix = cell_constraints

        # Set up 9x81 identity matrix
        eye = np.eye(9, 81)

        # Add row constraints
        row_constraints = np.tile(eye, (9, 1))
        for i in range(1, 9):
            row_constraints = np.vstack(
                (row_constraints, np.tile(np.roll(eye, 9 * i), (9, 1)))
            )

        self.matrix = np.hstack((self.matrix, row_constraints))

        # Add column constraints
        col_constraints = eye
        for i in range(1, 9):
            col_constraints = np.vstack((col_constraints, np.roll(eye, 9 * i)))
        col_constraints = np.tile(col_constraints, (9, 1))

        self.matrix = np.hstack((self.matrix, col_constraints))

        # Add block constraints
        tmp = np.tile(eye, (3, 1))
        for i in range(1, 3):
            tmp = np.vstack((tmp, np.tile(np.roll(eye, 9 * i), (3, 1))))

        tmp = np.tile(tmp, (3, 1))
        block_constraints = tmp
        for i in range(1, 3):
            block_constraints = np.vstack((block_constraints, np.roll(tmp, 9 * 3 * i)))

        self.matrix = np.hstack((self.matrix, block_constraints)).astype(dtype=int)

    def __str__(self) -> str:
        return "Algorithm X Sudoku Solver"

    def solve(self, sudoku: np.ndarray):
        assert sudoku.shape == (9, 9), "Input of invalid dimension."

        # Create mask to select active rows in constraints matrix
        mask = np.zeros(len(self.candidates), dtype=bool)
        for i in range(9):
            for j in range(9):
                if sudoku[i, j] == 0:
                    start_idx = 81 * i + 9 * j
                    mask[start_idx : (start_idx + 9)] = True
                else:
                    idx = 81 * i + 9 * j + sudoku[i, j] - 1
                    mask[idx] = True

        matrix = self.matrix[mask, :]

        solution = AlgorithmX.solve(matrix)

        if solution is None:
            return None

        # If we found a solution, fill out sudoku
        candidates = self.candidates[mask]
        for idx in solution:
            c = candidates[idx]
            sudoku[c.row - 1, c.col - 1] = c.num

        return sudoku

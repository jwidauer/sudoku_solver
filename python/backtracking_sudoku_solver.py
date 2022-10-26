import numpy as np


class BacktrackingSudokuSolver():
    def __str__(self) -> str:
        return "Backtracking Sudoku Solver"

    @staticmethod
    def solve(sudoku):
        sudoku = np.array(sudoku)
        if sudoku.shape != (9, 9):
            raise AttributeError('Input of non valid size.')

        return BacktrackingSudokuSolver.solve_sudoku(sudoku)
    
    @staticmethod
    def try_numbers(sudoku, row, col):
        for k in range(1, 10):
            if BacktrackingSudokuSolver.is_viable(sudoku, row, col, k):
                sudoku[row, col] = k
                solution = BacktrackingSudokuSolver.solve_sudoku(sudoku)
                if solution is not None:
                    return solution
                sudoku[row, col] = 0
        return None

    @staticmethod
    def solve_sudoku(sudoku):
        for i in range(9):
            for j in range(9):
                if sudoku[i, j] == 0:
                    return BacktrackingSudokuSolver.try_numbers(sudoku, i, j)
        return sudoku

    @staticmethod
    def is_viable(sudoku, row, col, num):
        # Check row viability
        if num in sudoku[:, col]:
            return False

        # Check column viability
        if num in sudoku[row, :]:
            return False

        # Check box viability
        box_row = (row // 3) * 3
        box_col = (col // 3) * 3

        if num in sudoku[box_row:(box_row + 3),
                         box_col:(box_col + 3)]:
            return False

        # Everything viable
        return True

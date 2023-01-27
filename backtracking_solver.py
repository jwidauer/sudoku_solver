class BacktrackingSudokuSolver:
    @staticmethod
    def solve(sudoku):
        sudoku = sudoku.to_array()
        if sudoku.shape != (9, 9):
            raise AttributeError("Input of non valid size.")

        return BacktrackingSudokuSolver.solve_sudoku(sudoku)

    @staticmethod
    def solve_sudoku(sudoku):
        for i in range(9):
            for j in range(9):
                if sudoku[i, j] == 0:
                    return BacktrackingSudokuSolver.try_numbers(sudoku, i, j)
        return sudoku.tolist()

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
    def is_viable(sudoku, row, col, num):
        # Check row viability
        if num in sudoku[:, col]:
            return False

        # Check column viability
        if num in sudoku[row, :]:
            return False

        # Check box viability
        box_row_start = (row // 3) * 3
        box_col_start = (col // 3) * 3
        box_row_end = box_row_start + 3
        box_col_end = box_col_start + 3

        if num in sudoku[box_row_start:box_row_end, box_col_start:box_col_end]:
            return False

        # Everything viable
        return True

import numpy as np


class Sudoku:
    def __init__(self, sudoku):
        if isinstance(sudoku, str):
            sudoku = Sudoku.from_string(sudoku)
        if isinstance(sudoku, list):
            sudoku = np.array(sudoku)

        assert isinstance(sudoku, np.ndarray)
        assert sudoku.shape == (9, 9)
        self._sudoku = sudoku

    def __str__(self):
        result = ""
        for row_idx, row in enumerate(self._sudoku):
            if row_idx % 3 == 0:
                result += " -----------------------------\n"
            for col_idx, col in enumerate(row):
                if col_idx % 3 == 0:
                    result += "|"
                char = str(col) if col != 0 else "."
                result += " " + char + " "
            result += "|\n"
        result += " -----------------------------\n"
        return result

    def to_array(self):
        return self._sudoku

    @staticmethod
    def from_string(sudoku_string):
        sudoku = np.zeros((9, 9), dtype=int)
        for i, char in enumerate(sudoku_string):
            if char in "123456789":
                sudoku[i // 9, i % 9] = int(char)
        return Sudoku(sudoku)

    def is_solved(self):
        if self._sudoku.shape != (9, 9):
            print("Sudoku has invalid shape.")
            return False

        correct_set = set(range(1, 10))

        # Check column constraints
        for i in range(9):
            if set(self._sudoku[:, i]) != correct_set:
                print("Column constraint violated")
                return False

        # Check row constraints
        for i in range(9):
            if set(self._sudoku[i, :]) != correct_set:
                print("Row constraint violated")
                return False

        # Check block constraints
        for i in range(3):
            for j in range(3):
                start_row = 3 * i
                start_col = 3 * j
                end_row = start_row + 3
                end_col = start_col + 3
                if (
                    set(self._sudoku[start_row:end_row, start_col:end_col].flatten())
                    != correct_set
                ):
                    print("Block constraint violated")
                    return False

        return True


if __name__ == "__main__":
    sudoku = Sudoku(
        np.array(
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]
        )
    )
    print(sudoku)

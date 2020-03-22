import numpy as np

def check_sudoku(sudoku):
    sudoku = np.array(sudoku)
    assert sudoku.shape == (9, 9), 'Sudoku has invalid shape.'

    correct_set = set(range(1, 10))

    # Check column constraints
    for i in range(9):
        if set(sudoku[:, i]) != correct_set:
            print('Column constraint violated')
            return False
    
    # Check row constraints
    for i in range(9):
        if set(sudoku[i, :]) != correct_set:
            print('Row constraint violated')
            return False
    
    # Check block constraints
    for i in range(3):
        for j in range(3):
            row_idx = 3 * i
            col_idx = 3 * j
            if set(sudoku[row_idx:row_idx + 3, col_idx:col_idx + 3].flatten()) != correct_set:
                print('Block constraint violated')
                return False
    
    return True

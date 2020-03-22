#!/usr/bin/python3

import pprint

from algorithm_x import AlgorithmXSudokuSolver
from check_sudoku import check_sudoku

pp = pprint.PrettyPrinter()

# sudoku = [[0, 0, 0, 0, 0, 0, 0, 0, 0],
#           [0, 0, 0, 0, 0, 3, 0, 8, 5],
#           [0, 0, 1, 0, 2, 0, 0, 0, 0],
#           [0, 0, 0, 5, 0, 7, 0, 0, 0],
#           [0, 0, 4, 0, 0, 0, 1, 0, 0],
#           [0, 9, 0, 0, 0, 0, 0, 0, 0],
#           [5, 0, 0, 0, 0, 0, 0, 7, 3],
#           [0, 0, 2, 0, 1, 0, 0, 0, 0],
#           [0, 0, 0, 0, 4, 0, 0, 0, 9]]

sudoku = [[5, 3, 0, 0, 7, 0, 0, 0, 0],
          [6, 0, 0, 1, 9, 5, 0, 0, 0],
          [0, 9, 8, 0, 0, 0, 0, 6, 0],
          [8, 0, 0, 0, 6, 0, 0, 0, 3],
          [4, 0, 0, 8, 0, 3, 0, 0, 1],
          [7, 0, 0, 0, 2, 0, 0, 0, 6],
          [0, 6, 0, 0, 0, 0, 2, 8, 0],
          [0, 0, 0, 4, 1, 9, 0, 0, 5],
          [0, 0, 0, 0, 8, 0, 0, 7, 9]]

solver = AlgorithmXSudokuSolver()
solution = solver.solve(sudoku)

if solution[0]:
    print(check_sudoku(solution[1]))
    pp.pprint(solution[1])
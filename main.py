#!/usr/bin/env python3

from timeit import default_timer as timer
import statistics

from algorithm_x_sudoku_solver import AlgorithmXSudokuSolver
from backtracking_solver import BacktrackingSudokuSolver
from sudoku import Sudoku

sudokus = []
with open("test_sudokus.txt") as f:
    sudokus = [Sudoku.from_string(sudoku) for sudoku in f]

print("Starting to solve!\n")

solver = AlgorithmXSudokuSolver()

runtimes = []
for sudoku in sudokus:
    print(sudoku)
    start = timer()
    solution = solver.solve(sudoku)
    end = timer()

    if not solution:
        print("No solution found!")
        print(sudoku)
        break

    sol = Sudoku(solution)
    if not sol.is_solved():
        print("Solution is not valid!")
        print(sol)
        break

    runtimes.append(end - start)
    print(f"Solved sudoku in {round((end - start) * 1e3, 3)}ms\n")

    # Only solve the first sudokus
    # if len(runtimes) == 2:
    #     break


average_runtime = sum(runtimes) / len(runtimes) * 1e3
std_runtime = statistics.pstdev(runtimes) * 1e3
max_runtime = max(runtimes) * 1e3
min_runtime = min(runtimes) * 1e3

print("Finished solving!")
print(f"Solved {len(runtimes)} sudokus.")
print(
    f"Runtime was:"
    f" avg: {round(average_runtime, 3)}(+/-{round(std_runtime, 3)})ms \t "
    f"max: {round(max_runtime, 3)}ms \t min: {round(min_runtime, 3)}ms"
)

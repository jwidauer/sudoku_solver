#!/usr/bin/python3

from timeit import default_timer as timer
import statistics

import numpy as np

from algorithm_x_sudoku_solver import AlgorithmXSudokuSolver
from backtracking_sudoku_solver import BacktrackingSudokuSolver
from check_sudoku import is_solved


def convert_to_grid(string: str) -> np.ndarray:
    grid = np.zeros((9, 9), dtype=int)
    for i, val in enumerate(string):
        if val in "123456789":
            grid[int(i / 9)][i % 9] = int(val)
    return np.array(grid)


f = open("../test_sudokus.txt")

sudokus: list[np.ndarray] = []
for sudoku in f:
    sudokus.append(convert_to_grid(sudoku))

print("Starting to solve!")

solver = AlgorithmXSudokuSolver()
print(f"Using {solver}")

runtimes = []
found_sol = 0
for idx, sudoku in enumerate(sudokus):
    print(f"\nSolving sudoku {idx + 1}/{len(sudokus)}")
    start = timer()
    solution = solver.solve(sudoku)
    end = timer()
    computation_time = end - start

    if solution is not None and is_solved(solution):
        print(f"Solution found in {round(computation_time * 1e3, 3)}ms")
        found_sol += 1
    else:
        print("No solution found")
        print("Solution:")
        print(solution)

    runtimes.append(computation_time)

    # if idx == 0:
    #     break

solved_percentage = int(found_sol / len(runtimes) * 100)

average_runtime = sum(runtimes) / len(runtimes) * 1e3
std_runtime = statistics.pstdev(runtimes) * 1e3
max_runtime = max(runtimes) * 1e3
min_runtime = min(runtimes) * 1e3

print(f"Found solution to {found_sol}/{len(runtimes)} ({solved_percentage}%) sudokus.")
print(
    f"Runtime was: avg: {round(average_runtime, 3)}(+/-{round(std_runtime, 3)})ms \t max: {round(max_runtime, 3)}ms \t min: {round(min_runtime, 3)}ms"
)

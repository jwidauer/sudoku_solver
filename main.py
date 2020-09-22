#!/usr/bin/python3

from timeit import default_timer as timer
import statistics

from algorithm_x_sudoku_solver import AlgorithmXSudokuSolver


def convert_to_grid(string):
    grid = [[0]*9]*9
    for i, val in enumerate(string):
        if val in "123456789":
            grid[int(i/9)][i % 9] = int(val)
    return grid


f = open("test_sudokus.txt")

sudokus = []
for sudoku in f:
    sudokus.append(convert_to_grid(sudoku))

print("Starting to solve!")

solver = AlgorithmXSudokuSolver()

runtimes = []
found_sol = 0
for sudoku in sudokus:
    start = timer()
    solution = solver.solve(sudoku)
    end = timer()
    found_sol += 1
    runtimes.append(end - start)

solved_percentage = int(found_sol / len(runtimes) * 100)

average_runtime = sum(runtimes) / len(runtimes) * 1e3
std_runtime = statistics.pstdev(runtimes) * 1e3
max_runtime = max(runtimes) * 1e3
min_runtime = min(runtimes) * 1e3

print(
    f"Found solution to {found_sol}/{len(runtimes)} ({solved_percentage}%) sudokus.")
print(f"Runtime was: avg: {round(average_runtime, 3)}(+/-{round(std_runtime, 3)})ms \t max: {round(max_runtime, 3)}ms \t min: {round(min_runtime, 3)}ms")

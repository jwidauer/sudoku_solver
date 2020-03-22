import numpy as np
from typing import TypeVar, List

from check_sudoku import check_sudoku

T = TypeVar('T')

class Node:
    def __init__(self, name = None, l = None, r = None, u = None, d = None, c = None):
        self.name = name
        self.l = l
        self.r = r
        self.u = u
        self.d = d
        self.c = c

class HeaderNode(Node):
    def __init__(self, l = None, r = None, u = None, d = None, c = None):
        super().__init__(None, l, r, u, d, c)
        self.size = 0

class AlgorithmX():
    def __init__(self, matrix: np.array):
        self.h = HeaderNode()
        self.h.u = self.h.d = self.h.r = self.h.l = self.h

        # Create the column headers
        headers = [self.h]
        for _ in range(matrix.shape[1]):
            new_node = HeaderNode(l = headers[-1], r = self.h)
            new_node.l.r = new_node
            new_node.r.l = new_node
            new_node.u = new_node.d = new_node.c = new_node
            headers.append(new_node)
        
        # Create linked list "matrix"
        for row_num, row in enumerate(matrix):
            prev_node = None
            for idx, val in enumerate(row):
                if val != 0:
                    new_node = Node(name = row_num)
                    self.insert_above(headers[idx + 1], new_node)

                    if prev_node == None:
                        # First node this row
                        new_node.r = new_node
                        new_node.l = new_node
                    else:
                        new_node.r = prev_node.r
                        new_node.l = prev_node
                        new_node.r.l = new_node
                        new_node.l.r = new_node
                    prev_node = new_node

    def insert_above(self, node: Node, new_node: Node):
        new_node.u = node.u
        new_node.d = node
        new_node.c = node.c
        new_node.u.d = new_node
        new_node.d.u = new_node
        new_node.c.size += 1
    
    def insert_below(self, node: Node, new_node: Node):
        new_node.u = node
        new_node.d = node.d
        new_node.c = node.c
        new_node.u.d = new_node
        new_node.d.u = new_node
        new_node.c.size += 1
    
    def choose_column(self):
        # Chooses column with smallest number of nodes
        min_size = float('inf')
        min_node = self.h

        cur_node = self.h.r
        while cur_node != self.h:
            if cur_node.size < min_size:
                min_node = cur_node
                min_size = cur_node.size
            cur_node = cur_node.r
        return min_node

    def cover_column(self, column: HeaderNode):
        column.r.l = column.l
        column.l.r = column.r

        col_node = column.d
        while col_node != column:
            row_node = col_node.r
            while row_node != col_node:
                row_node.d.u = row_node.u
                row_node.u.d = row_node.d
                row_node.c.size -= 1
                row_node = row_node.r
            col_node = col_node.d

    def uncover_column(self, column: HeaderNode):
        col_node = column.u
        while col_node != column:
            row_node = col_node.l
            while row_node != col_node:
                row_node.c.size += 1
                row_node.d.u = row_node
                row_node.u.d = row_node
                row_node = row_node.l
            col_node = col_node.u
        
        column.r.l = column
        column.l.r = column
    
    def search(self):
        # Find the exact cover for the set up matrix
        # Returns whether a solution was found and 
        # a list of row indices for the solution
        if self.h.r == self.h:
            return (True, [])
        
        column = self.choose_column()
        self.cover_column(column)

        col_node = column.d
        while col_node != column:
            O = col_node

            row_node = col_node.r
            while row_node != col_node:
                self.cover_column(row_node.c)
                row_node = row_node.r
            
            sol = self.search()
            if sol[0]:
                sol[1].append(O.name)
                return (True, sol[1])
            
            row_node = col_node.l
            while row_node != col_node:
                self.uncover_column(row_node.c)
                row_node = row_node.l
            
            col_node = col_node.d
        
        self.uncover_column(column)
        return (False, [])

class Candidate:
    row: int
    col: int
    num: int

    def __init__(self, row, col, num):
        self.row = row
        self.col = col
        self.num = num
    
    def __str__(self):
        return 'r' + str(self.row) + 'c' + str(self.col) + '#' + str(self.num)

class AlgorithmXSudokuSolver:
    def __init__(self):
        # Set up candidate list
        self.candidates = []
        for row in range(1, 10):
            for col in range(1, 10):
                for num in range(1, 10):
                    self.candidates.append(Candidate(row, col, num))

        self.candidates = np.array(self.candidates)

        ## Set up constraints matrix
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
            row_constraints = np.vstack((row_constraints, np.tile(np.roll(eye, 9 * i), (9, 1))))
        
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

    def solve(self, sudoku):
        sudoku = np.array(sudoku)
        assert sudoku.shape == (9, 9), 'Input of invalid dimension.'

        # Create mask to select active rows in constraints matrix
        mask = np.zeros(len(self.candidates), dtype=bool)
        for i in range(9):
            for j in range(9):
                if sudoku[i, j] == 0:
                    start_idx = 81 * i + 9 * j
                    mask[start_idx:(start_idx + 9)] = True
                else:
                    idx = 81 * i + 9 * j + sudoku[i, j] - 1
                    mask[idx] = True
        
        matrix = self.matrix[mask, :]

        algorithm_x = AlgorithmX(matrix)
        solution = algorithm_x.search()

        # If we found a solution, fill out sudoku
        if solution[0]:
            candidates = self.candidates[mask]
            for sol in candidates[solution[1]]:
                sudoku[sol.row - 1, sol.col - 1] = sol.num
        
        return (solution[0], sudoku.tolist())

import numpy as np
from typing import override


class Node:
    def __init__(self, name=None, l=None, r=None, u=None, d=None, c=None):
        self.row_num: int | None = name
        self.l: Node | None = l
        self.r: Node | None = r
        self.u: Node | None = u
        self.d: Node | None = d
        self.c: Node | None = c

    @override
    def __str__(self):
        L = self.l.row_num if self.l else None
        R = self.r.row_num if self.r else None
        U = self.u.row_num if self.u else None
        D = self.d.row_num if self.d else None
        return f"Node({self.row_num}): L={L}, R={R}, U={U}, D={D}"


class HeaderNode(Node):
    def __init__(self, l=None, r=None, u=None, d=None, c=None):
        super().__init__(None, l, r, u, d, c)
        self.size: int = 0


class AlgorithmX:
    @staticmethod
    def solve(matrix: np.ndarray):
        h = AlgorithmX.setup_grid(matrix)
        return AlgorithmX.search(h)

    @staticmethod
    def setup_grid(matrix: np.ndarray):
        h = HeaderNode()
        h.u = h.d = h.r = h.l = h

        # Create the column headers
        headers = [h]
        for _ in range(matrix.shape[1]):
            new_node = HeaderNode(l=headers[-1], r=h)
            new_node.l.r = new_node
            new_node.r.l = new_node
            new_node.u = new_node.d = new_node.c = new_node
            headers.append(new_node)

        count = 0

        # Create linked list "matrix"
        for row_idx, row in enumerate(matrix):
            prev_node = None
            for col_idx, val in enumerate(row):
                if val != 0:
                    count += 1
                    new_node = Node(name=row_idx)
                    AlgorithmX.insert_above(headers[col_idx + 1], new_node)

                    if prev_node is None:
                        # First node this row
                        new_node.r = new_node
                        new_node.l = new_node
                    else:
                        new_node.r = prev_node.r
                        new_node.l = prev_node
                        new_node.r.l = new_node
                        new_node.l.r = new_node

                    prev_node = new_node

        return h

    @staticmethod
    def insert_above(node: Node, new_node: Node):
        new_node.u = node.u
        new_node.d = node
        new_node.c = node.c
        new_node.u.d = new_node
        new_node.d.u = new_node
        new_node.c.size += 1

    @staticmethod
    def insert_below(node: Node, new_node: Node):
        new_node.u = node
        new_node.d = node.d
        new_node.c = node.c
        new_node.u.d = new_node
        new_node.d.u = new_node
        new_node.c.size += 1

    @staticmethod
    def choose_column(h: HeaderNode):
        # Chooses column with smallest number of nodes
        min_size = float("inf")
        min_node = h

        cur_node = h.r
        while cur_node != h:
            if cur_node.size < min_size:
                min_node = cur_node
                min_size = cur_node.size
            cur_node = cur_node.r
        return min_node

    @staticmethod
    def cover_column(column: HeaderNode):
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

    @staticmethod
    def uncover_column(column: HeaderNode):
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

    @staticmethod
    def search(h):
        # Find the exact cover for the set up matrix
        # Returns whether a solution was found and
        # a list of row indices for the solution
        if h.r == h:
            return []

        column = AlgorithmX.choose_column(h)
        AlgorithmX.cover_column(column)

        col_node = column.d
        while col_node != column:
            O = col_node

            row_node = col_node.r
            while row_node != col_node:
                AlgorithmX.cover_column(row_node.c)
                row_node = row_node.r

            sol = AlgorithmX.search(h)
            if sol is not None:
                sol.append(O.row_num)
                return sol

            row_node = col_node.l
            while row_node != col_node:
                AlgorithmX.uncover_column(row_node.c)
                row_node = row_node.l

            col_node = col_node.d

        AlgorithmX.uncover_column(column)
        return None

import numpy as np


class Node:
    def __init__(self, name=None, l=None, r=None, u=None, d=None, c=None):
        self.name = name
        self.l = l
        self.r = r
        self.u = u
        self.d = d
        self.c = c


class HeaderNode(Node):
    def __init__(self, l=None, r=None, u=None, d=None, c=None):
        super().__init__(None, l, r, u, d, c)
        self.size = 0


class AlgorithmX:
    @staticmethod
    def solve(matrix: np.array):
        h = AlgorithmX.setup_grid(matrix)
        return AlgorithmX.search(h)

    @staticmethod
    def setup_grid(matrix: np.array):
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

        # Create linked list "matrix"
        for row_num, row in enumerate(matrix):
            prev_node = None
            for idx, val in enumerate(row):
                if val != 0:
                    new_node = Node(name=row_num)
                    AlgorithmX.insert_above(headers[idx + 1], new_node)

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
    def choose_column(h):
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
                sol.append(O.name)
                return sol

            row_node = col_node.l
            while row_node != col_node:
                AlgorithmX.uncover_column(row_node.c)
                row_node = row_node.l

            col_node = col_node.d

        AlgorithmX.uncover_column(column)
        return None

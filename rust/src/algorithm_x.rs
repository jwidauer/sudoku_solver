use ndarray::prelude::*;

#[derive(Clone, Debug)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    row: usize, // row idx of the original matrix
    col: usize, // idx of the corresponding col header node
}

impl Node {
    fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            row: 0,
            col: 0,
        }
    }
}

pub struct NodeGrid {
    nodes: Vec<Node>,
    col_counts: Vec<usize>,
}

impl NodeGrid {
    const ROOT: usize = 0;

    pub fn new(matrix: Array2<bool>) -> Self {
        let n_cols = matrix.dim().1;

        let mut nodes: Vec<Node> = Vec::with_capacity(n_cols + 1);
        let col_counts = vec![0usize; n_cols + 1];

        // Add the "root" node
        nodes.push(Node::new());

        // Set up header nodes
        for i in 1..=n_cols {
            let new_node = Node {
                left: i - 1,
                right: 0,
                up: i,
                down: i,
                row: 0,
                col: i,
            };

            nodes.last_mut().expect("We know it's not empty").right = i;

            nodes.push(new_node);
        }

        nodes.first_mut().expect("We know it's not empty").left = nodes.len() - 1;

        let mut grid = NodeGrid { nodes, col_counts };

        // Convert matrix into "grid"
        for (row_idx, row) in matrix.rows().into_iter().enumerate() {
            let mut first_in_row = None;
            for (col_idx, &element) in row.into_iter().enumerate() {
                if !element {
                    continue;
                }

                let idx = grid.insert_above(col_idx + 1, row_idx);
                match first_in_row {
                    None => {
                        grid.nodes[idx].right = idx;
                        grid.nodes[idx].left = idx;
                        first_in_row = Some(idx);
                    }
                    Some(first_idx) => {
                        let left_idx = idx - 1;
                        grid.nodes[idx].left = left_idx;
                        grid.nodes[idx].right = first_idx;

                        grid.nodes[first_idx].left = idx;
                        grid.nodes[left_idx].right = idx;
                    }
                }
            }
        }

        grid
    }

    fn node(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }

    fn node_mut(&mut self, idx: usize) -> &mut Node {
        &mut self.nodes[idx]
    }

    fn count(&self, col_idx: usize) -> usize {
        self.col_counts[col_idx]
    }

    fn count_mut(&mut self, col_idx: usize) -> &mut usize {
        &mut self.col_counts[col_idx]
    }

    fn insert_above(&mut self, hdr_idx: usize, row_idx: usize) -> usize {
        let new_idx = self.nodes.len();

        // Update the node above the header node to point to new node
        let above_idx = self.nodes[hdr_idx].up;
        self.nodes[above_idx].down = new_idx;

        // Insert the new node
        let hdr_node = &mut self.nodes[hdr_idx];
        let new_node = Node {
            left: 0,
            right: 0,
            up: above_idx,
            down: hdr_idx,
            row: row_idx,
            col: hdr_node.col,
        };
        hdr_node.up = new_idx;
        self.col_counts[hdr_node.col] += 1;

        self.nodes.push(new_node);

        new_idx
    }

    fn choose_column(&self) -> usize {
        let mut min_count = usize::MAX;
        let mut min_node = 0;

        let mut cur_node = self.node(Self::ROOT).right;
        while cur_node != Self::ROOT {
            if self.count(self.node(cur_node).col) < min_count {
                min_count = self.count(self.node(cur_node).col);
                min_node = cur_node;
            }
            cur_node = self.node(cur_node).right;
        }
        min_node
    }

    fn cover_column(&mut self, col: usize) {
        let Node {
            left, right, down, ..
        } = *self.node(col);
        self.node_mut(right).left = left;
        self.node_mut(left).right = right;

        let mut col_node = down;
        while col_node != col {
            let mut row_node = self.node(col_node).right;
            while row_node != col_node {
                let Node {
                    up,
                    down,
                    col: row_col,
                    ..
                } = *self.node(row_node);
                self.node_mut(down).up = up;
                self.node_mut(up).down = down;
                *self.count_mut(row_col) -= 1;
                row_node = self.node(row_node).right;
            }
            col_node = self.node(col_node).down;
        }
    }

    fn uncover_column(&mut self, col: usize) {
        let Node {
            left, right, up, ..
        } = *self.node(col);

        let mut col_node = up;
        while col_node != col {
            let mut row_node = self.node(col_node).left;
            while row_node != col_node {
                let Node {
                    up,
                    down,
                    col: row_col,
                    ..
                } = *self.node(row_node);
                self.node_mut(down).up = row_node;
                self.node_mut(up).down = row_node;
                *self.count_mut(row_col) += 1;
                row_node = self.node(row_node).left;
            }
            col_node = self.node(col_node).up;
        }

        self.node_mut(right).left = col;
        self.node_mut(left).right = col;
    }

    pub fn search(&mut self) -> Option<Vec<usize>> {
        if self.node(Self::ROOT).right == Self::ROOT {
            return Some(Vec::new());
        }

        let col = self.choose_column();
        self.cover_column(col);

        let mut col_node = self.node(col).down;
        while col_node != col {
            let o = col_node;

            let mut row_node = self.node(col_node).right;
            while row_node != col_node {
                let row_col = self.node(row_node).col;
                self.cover_column(row_col);
                row_node = self.node(row_node).right;
            }

            if let Some(mut result) = self.search() {
                result.push(self.node(o).row);
                return Some(result);
            }

            row_node = self.node(col_node).left;
            while row_node != col_node {
                self.uncover_column(self.node(row_node).col);
                row_node = self.node(row_node).left;
            }

            col_node = self.node(col_node).down;
        }

        self.uncover_column(col);
        None
    }
}

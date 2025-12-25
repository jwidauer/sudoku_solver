#[derive(Clone, Debug)]
struct Node {
    left: u16,
    right: u16,
    up: u16,
    down: u16,
    /// row idx of the original matrix
    row: u16,
    /// idx of the corresponding col header node
    col: u16,
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
    const ROOT: u16 = 0;

    pub fn from_sparse_matrix(sparse_mat: &[[u16; 4]], n_total_cols: usize) -> Self {
        let n_rows = sparse_mat.len();
        let n_cols = n_total_cols;

        let mut nodes: Vec<Node> = Vec::with_capacity(n_cols + 1 + 4 * n_rows);
        let col_counts = vec![0usize; n_cols + 1];

        // Add the "root" node
        nodes.push(Node::new());

        // Set up header nodes
        for i in 1..=n_cols {
            let i = i as u16;
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

        nodes.first_mut().expect("We know it's not empty").left = (nodes.len() - 1) as u16;

        let mut grid = NodeGrid { nodes, col_counts };

        // Convert sparse matrix into "grid"
        for (row_idx, row) in sparse_mat.iter().enumerate() {
            let row_idx = row_idx as u16;
            let mut first_in_row = None;
            for &col in row.iter() {
                grid.insert_new(row_idx, col, &mut first_in_row);
            }
        }

        grid
    }

    fn insert_new(&mut self, row_idx: u16, col_idx: u16, first_in_row: &mut Option<u16>) {
        let idx = self.insert_above(col_idx + 1, row_idx);
        match *first_in_row {
            None => {
                self.node_mut(idx).right = idx;
                self.node_mut(idx).left = idx;
                *first_in_row = Some(idx);
            }
            Some(first_idx) => {
                let left_idx = idx - 1;
                self.node_mut(idx).left = left_idx;
                self.node_mut(idx).right = first_idx;

                self.node_mut(first_idx).left = idx;
                self.node_mut(left_idx).right = idx;
            }
        }
    }

    #[inline(always)]
    fn node(&self, idx: u16) -> &Node {
        unsafe { self.nodes.get_unchecked(idx as usize) }
    }

    #[inline(always)]
    fn node_mut(&mut self, idx: u16) -> &mut Node {
        unsafe { self.nodes.get_unchecked_mut(idx as usize) }
    }

    #[inline(always)]
    fn count(&self, col_idx: u16) -> usize {
        unsafe { *self.col_counts.get_unchecked(col_idx as usize) }
    }

    #[inline(always)]
    fn count_mut(&mut self, col_idx: u16) -> &mut usize {
        unsafe { self.col_counts.get_unchecked_mut(col_idx as usize) }
    }

    #[inline(always)]
    fn inc_count(counts: &mut [usize], col_idx: u16) {
        unsafe {
            *counts.get_unchecked_mut(col_idx as usize) += 1;
        }
    }

    #[inline(always)]
    fn insert_above(&mut self, hdr_idx: u16, row_idx: u16) -> u16 {
        let new_idx = self.nodes.len() as u16;

        // Update the node above the header node to point to new node
        let above_idx = self.node(hdr_idx).up;
        self.node_mut(above_idx).down = new_idx;

        // Insert the new node
        let hdr_node = &mut self.nodes[hdr_idx as usize];
        let new_node = Node {
            left: 0,
            right: 0,
            up: above_idx,
            down: hdr_idx,
            row: row_idx,
            col: hdr_node.col,
        };
        hdr_node.up = new_idx;
        Self::inc_count(&mut self.col_counts, hdr_node.col);

        self.nodes.push(new_node);

        new_idx
    }

    fn choose_column(&self) -> u16 {
        let mut min_count = usize::MAX;
        let mut min_node = 0;

        let mut cur_node = self.node(Self::ROOT).right;
        while cur_node != Self::ROOT {
            let count = self.count(cur_node);
            (min_count, min_node) = if count < min_count {
                (count, cur_node)
            } else {
                (min_count, min_node)
            };
            cur_node = self.node(cur_node).right;
        }
        min_node
    }

    fn cover_column(&mut self, col: u16) {
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

    fn uncover_column(&mut self, col: u16) {
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

    pub fn search(&mut self) -> Option<Vec<u16>> {
        if self.node(Self::ROOT).right == Self::ROOT {
            return Some(Vec::new());
        }

        let col = self.choose_column();
        self.cover_column(col);

        let mut col_node = self.node(col).down;
        while col_node != col {
            let origin = col_node;

            let mut row_node = self.node(col_node).right;
            while row_node != col_node {
                let row_col = self.node(row_node).col;
                self.cover_column(row_col);
                row_node = self.node(row_node).right;
            }

            if let Some(mut result) = self.search() {
                result.push(self.node(origin).row);
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

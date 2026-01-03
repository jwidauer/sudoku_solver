use super::solver::NR_CONSTRAINTS;

struct UncheckedIndexVec<T>(Vec<T>);

impl<T: Default + Clone> UncheckedIndexVec<T> {
    fn new(size: usize) -> Self {
        Self(vec![T::default(); size])
    }

    #[inline(always)]
    fn get(&self, idx: u16) -> &T {
        debug_assert!((idx as usize) < self.0.len());
        unsafe { self.0.get_unchecked(idx as usize) }
    }

    #[inline(always)]
    fn get_mut(&mut self, idx: u16) -> &mut T {
        debug_assert!((idx as usize) < self.0.len());
        unsafe { self.0.get_unchecked_mut(idx as usize) }
    }
}

struct NodeList {
    left: UncheckedIndexVec<u16>,
    right: UncheckedIndexVec<u16>,
    up: UncheckedIndexVec<u16>,
    down: UncheckedIndexVec<u16>,
    row: UncheckedIndexVec<u16>,
    col: UncheckedIndexVec<u16>,
}

impl NodeList {
    fn new(capacity: usize) -> Self {
        Self {
            left: UncheckedIndexVec::new(capacity),
            right: UncheckedIndexVec::new(capacity),
            up: UncheckedIndexVec::new(capacity),
            down: UncheckedIndexVec::new(capacity),
            row: UncheckedIndexVec::new(capacity),
            col: UncheckedIndexVec::new(capacity),
        }
    }

    #[inline(always)]
    fn left(&self, idx: u16) -> u16 {
        *self.left.get(idx)
    }

    #[inline(always)]
    fn left_mut(&mut self, idx: u16) -> &mut u16 {
        self.left.get_mut(idx)
    }

    #[inline(always)]
    fn right(&self, idx: u16) -> u16 {
        *self.right.get(idx)
    }

    #[inline(always)]
    fn right_mut(&mut self, idx: u16) -> &mut u16 {
        self.right.get_mut(idx)
    }

    #[inline(always)]
    fn up(&self, idx: u16) -> u16 {
        *self.up.get(idx)
    }

    #[inline(always)]
    fn up_mut(&mut self, idx: u16) -> &mut u16 {
        self.up.get_mut(idx)
    }

    #[inline(always)]
    fn down(&self, idx: u16) -> u16 {
        *self.down.get(idx)
    }

    #[inline(always)]
    fn down_mut(&mut self, idx: u16) -> &mut u16 {
        self.down.get_mut(idx)
    }

    #[inline(always)]
    fn row(&self, idx: u16) -> u16 {
        *self.row.get(idx)
    }

    #[inline(always)]
    fn col(&self, idx: u16) -> u16 {
        *self.col.get(idx)
    }
}

pub struct NodeGrid {
    nodes: NodeList,
    col_counts: [u8; NR_CONSTRAINTS + 1],
}

impl NodeGrid {
    const ROOT: u16 = 0;

    pub fn from_sparse_matrix(sparse_mat: &[[u16; 4]], n_total_cols: usize) -> Self {
        let n_rows = sparse_mat.len();
        let n_cols = n_total_cols;
        let n_hdr_nodes = n_cols + 1; // +1 for root node

        let mut nodes = NodeList::new(n_hdr_nodes + 4 * n_rows);

        // Set up header nodes
        for i in 1..=n_cols {
            let i = i as u16;
            // Link header nodes in a circular doubly linked list
            *nodes.left_mut(i) = i - 1;
            *nodes.right_mut(i - 1) = i;

            *nodes.up_mut(i) = i;
            *nodes.down_mut(i) = i;
            *nodes.col.get_mut(i) = i;
        }

        *nodes.left_mut(0) = (n_hdr_nodes - 1) as u16;

        let mut grid = NodeGrid {
            nodes,
            col_counts: [0; 325],
        };

        // Convert sparse matrix into "grid"
        let mut new_idx = n_hdr_nodes as u16;
        for (row_idx, row) in sparse_mat.iter().enumerate() {
            let row_idx = row_idx as u16;

            let mut first_in_row = None;
            for &col in row.iter() {
                grid.insert_new(new_idx, col, row_idx, &mut first_in_row);
                new_idx += 1;
            }
        }

        grid
    }

    #[inline(always)]
    fn insert_new(&mut self, idx: u16, hdr: u16, row_idx: u16, first_in_row: &mut Option<u16>) {
        self.insert_above(idx, hdr + 1, row_idx);
        match *first_in_row {
            None => {
                *self.nodes.right_mut(idx) = idx;
                *self.nodes.left_mut(idx) = idx;
                *first_in_row = Some(idx);
            }
            Some(first_idx) => {
                let left_idx = idx - 1;
                *self.nodes.left_mut(idx) = left_idx;
                *self.nodes.right_mut(idx) = first_idx;

                *self.nodes.left_mut(first_idx) = idx;
                *self.nodes.right_mut(left_idx) = idx;
            }
        }
    }

    #[inline(always)]
    fn insert_above(&mut self, new_idx: u16, hdr: u16, row_idx: u16) {
        // Update the node above the header node to point to new node
        let above = self.nodes.up(hdr);
        *self.nodes.down_mut(above) = new_idx;
        *self.nodes.up_mut(hdr) = new_idx;

        // Insert the new node
        *self.nodes.up_mut(new_idx) = above;
        *self.nodes.down_mut(new_idx) = hdr;
        *self.nodes.row.get_mut(new_idx) = row_idx;
        *self.nodes.col.get_mut(new_idx) = hdr;

        self.inc_count(hdr);
    }

    #[inline(always)]
    fn count(&self, col_idx: u16) -> u8 {
        unsafe { *self.col_counts.get_unchecked(col_idx as usize) }
    }

    #[inline(always)]
    fn count_mut(&mut self, col_idx: u16) -> &mut u8 {
        unsafe { self.col_counts.get_unchecked_mut(col_idx as usize) }
    }

    #[inline(always)]
    fn inc_count(&mut self, col_idx: u16) {
        *self.count_mut(col_idx) += 1;
    }

    #[inline(always)]
    fn dec_count(&mut self, col_idx: u16) {
        *self.count_mut(col_idx) -= 1;
    }

    #[inline(always)]
    fn choose_column(&self) -> u16 {
        let mut min_count = u8::MAX;
        let mut min_node = 0;

        let mut fwd_node = self.nodes.right(Self::ROOT);
        let mut bwd_node = self.nodes.left(Self::ROOT);
        while fwd_node < bwd_node && min_count != 0 {
            let count = self.count(fwd_node);
            if count < min_count {
                min_count = count;
                min_node = fwd_node;
            }
            let count = self.count(bwd_node);
            if count < min_count {
                min_count = count;
                min_node = bwd_node;
            }
            fwd_node = self.nodes.right(fwd_node);
            bwd_node = self.nodes.left(bwd_node);
        }
        min_node
    }

    #[inline(always)]
    fn cover_column(&mut self, col: u16) {
        // Remove the column header from the header list
        let left = self.nodes.left(col);
        let right = self.nodes.right(col);
        *self.nodes.left_mut(right) = left;
        *self.nodes.right_mut(left) = right;

        // Remove all rows in this column
        let mut col_node = self.nodes.down(col);
        while col_node != col {
            // Remove all nodes in this row
            let mut row_node = self.nodes.right(col_node);
            while row_node != col_node {
                let up = self.nodes.up(row_node);
                let down = self.nodes.down(row_node);
                *self.nodes.up_mut(down) = up;
                *self.nodes.down_mut(up) = down;

                let row_node_col = self.nodes.col(row_node);
                self.dec_count(row_node_col);
                row_node = self.nodes.right(row_node);
            }
            col_node = self.nodes.down(col_node);
        }
    }

    #[inline(always)]
    fn uncover_column(&mut self, col: u16) {
        let left = self.nodes.left(col);
        let right = self.nodes.right(col);

        let mut col_node = self.nodes.up(col);
        while col_node != col {
            let mut row_node = self.nodes.left(col_node);
            while row_node != col_node {
                let up = self.nodes.up(row_node);
                let down = self.nodes.down(row_node);
                *self.nodes.up_mut(down) = row_node;
                *self.nodes.down_mut(up) = row_node;

                let row_node_col = self.nodes.col(row_node);
                self.inc_count(row_node_col);
                row_node = self.nodes.left(row_node);
            }
            col_node = self.nodes.up(col_node);
        }

        *self.nodes.left_mut(right) = col;
        *self.nodes.right_mut(left) = col;
    }

    #[inline(always)]
    fn cover_row(&mut self, origin: u16) {
        let mut row_node = self.nodes.right(origin);
        while row_node != origin {
            let row_node_col = self.nodes.col(row_node);
            self.cover_column(row_node_col);
            row_node = self.nodes.right(row_node);
        }
    }

    #[inline(always)]
    fn uncover_row(&mut self, origin: u16) {
        let mut row_node = self.nodes.left(origin);
        while row_node != origin {
            let row_node_col = self.nodes.col(row_node);
            self.uncover_column(row_node_col);
            row_node = self.nodes.left(row_node);
        }
    }

    // A recursive version of the algorithm X search
    // pub fn search(&mut self) -> Option<Vec<u16>> {
    //     if self.right(Self::ROOT) == Self::ROOT {
    //         return Some(Vec::new());
    //     }
    //
    //     let col_hdr = self.choose_column();
    //     self.cover_column(col_hdr);
    //
    //     let mut col_node = self.down(col_hdr);
    //     while col_node != col_hdr {
    //         self.cover_row(col_node);
    //
    //         if let Some(mut result) = self.search() {
    //             result.push(self.row(col_node));
    //             return Some(result);
    //         }
    //
    //         self.uncover_row(col_node);
    //
    //         col_node = self.down(col_node);
    //     }
    //
    //     self.uncover_column(col_hdr);
    //     None
    // }

    // A non recursive version of search
    pub fn search(&mut self) -> Option<Vec<u16>> {
        let mut stack: Vec<(u16, u16)> = Vec::with_capacity(128); // (col_hdr, col_node)

        loop {
            if self.nodes.right(Self::ROOT) == Self::ROOT {
                let result = stack
                    .into_iter()
                    .map(|(_, col_node)| self.nodes.row(col_node))
                    .collect();
                return Some(result);
            }

            let col_hdr = self.choose_column();
            self.cover_column(col_hdr);

            let col_node = self.nodes.down(col_hdr);
            if col_node != col_hdr {
                // Found a row to cover
                self.cover_row(col_node);
                stack.push((col_hdr, col_node));

                continue;
            }

            // Backtrack
            self.uncover_column(col_hdr);
            while let Some((prev_col_hdr, prev_col_node)) = stack.pop() {
                self.uncover_row(prev_col_node);
                let next_col_node = self.nodes.down(prev_col_node);
                if next_col_node != prev_col_hdr {
                    // Found the next row to cover
                    self.cover_row(next_col_node);
                    stack.push((prev_col_hdr, next_col_node));
                    break;
                }
                self.uncover_column(prev_col_hdr);
            }
            if stack.is_empty() {
                return None; // No more options to backtrack
            }
        }
    }
}

/// Node in the dancing links structure.
/// For column headers, `col == self_index` and `row == usize::MAX`.
#[derive(Clone, Copy, Debug)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    col: usize, // index of column header node
    row: usize, // row id (user-provided), usize::MAX for headers/root
}

/// DLX with secondary columns:
/// - Primary columns are in the root's horizontal list (must be covered).
/// - Secondary columns are NOT in root's list (optional, but enforced at most once).
pub struct Dlx {
    nodes: Vec<Node>,

    root: usize,               // root header node index (0)
    col_size: Vec<usize>,      // number of 1s in each column
    col_is_primary: Vec<bool>, // primary vs secondary
    col_head: Vec<usize>,      // header node index per column (points into nodes)
}

impl Dlx {
    /// Create a DLX instance with `num_primary` primary columns and `num_secondary` secondary columns.
    pub fn new(num_primary: usize, num_secondary: usize) -> Self {
        let root = 0usize;

        // root node
        let mut nodes = Vec::new();
        nodes.push(Node {
            left: root,
            right: root,
            up: root,
            down: root,
            col: root,
            row: usize::MAX,
        });

        let num_cols = num_primary + num_secondary;

        let col_size = vec![0usize; num_cols];
        let mut col_is_primary = vec![false; num_cols];
        let mut col_head = vec![0usize; num_cols];

        // Create column headers as nodes[1..=num_cols].
        // Primary headers are linked into root's horizontal list.
        // Secondary headers are left isolated horizontally (left=right=self).
        for c in 0..num_cols {
            let idx = nodes.len();
            col_head[c] = idx;

            col_is_primary[c] = c < num_primary;

            nodes.push(Node {
                left: idx,
                right: idx,
                up: idx,
                down: idx,
                col: idx,        // header's column is itself
                row: usize::MAX, // mark header
            });

            if col_is_primary[c] {
                // Insert header idx into root's horizontal list (to the left of root).
                let left_of_root = nodes[root].left;
                nodes[idx].right = root;
                nodes[idx].left = left_of_root;
                nodes[left_of_root].right = idx;
                nodes[root].left = idx;
            }
        }

        Self {
            nodes,
            root,
            col_size,
            col_is_primary,
            col_head,
        }
    }

    /// Add a row with given `row_id`, containing 1s in the specified columns.
    /// Columns are numbered 0..(num_primary+num_secondary).
    pub fn add_row(&mut self, row_id: usize, cols: &[usize]) {
        debug_assert!(!cols.is_empty());

        // Create one node per (row, col) 1-entry and link them in a circular row list.
        let mut row_nodes: Vec<usize> = Vec::with_capacity(cols.len());

        for &c in cols {
            let col_header = self.col_head[c];
            let nidx = self.nodes.len();

            // New node; temporarily self-linked horizontally, will fix later.
            self.nodes.push(Node {
                left: nidx,
                right: nidx,
                up: 0,
                down: 0,
                col: col_header,
                row: row_id,
            });

            // Insert vertically into column (at bottom, just above header).
            let up = self.nodes[col_header].up;
            self.nodes[nidx].down = col_header;
            self.nodes[nidx].up = up;
            self.nodes[up].down = nidx;
            self.nodes[col_header].up = nidx;

            self.col_size[c] += 1;

            row_nodes.push(nidx);
        }

        // Link horizontally in a circle.
        for i in 0..row_nodes.len() {
            let a = row_nodes[i];
            let b = row_nodes[(i + 1) % row_nodes.len()];
            self.nodes[a].right = b;
            self.nodes[b].left = a;
        }
    }

    /// Solve; returns one solution as a list of row_ids, or None if unsatisfiable.
    pub fn solve_one(&mut self) -> Option<Vec<usize>> {
        let mut out = Vec::new();
        if self.search(&mut out) {
            Some(out)
        } else {
            None
        }
    }

    // ---------------- internal DLX ops ----------------

    /// Choose a primary column with minimum size (heuristic).
    fn choose_column(&self) -> Option<usize> {
        let mut c = self.nodes[self.root].right;
        if c == self.root {
            return None; // no primary columns left => solved
        }

        let mut best = c;
        let mut best_size = self.col_size[self.col_index_from_head(c)];

        c = self.nodes[c].right;
        while c != self.root {
            let sz = self.col_size[self.col_index_from_head(c)];
            if sz < best_size {
                best = c;
                best_size = sz;
                if best_size == 0 {
                    break;
                }
            }
            c = self.nodes[c].right;
        }

        Some(best)
    }

    /// Convert a column header node index -> column number (0..num_cols).
    fn col_index_from_head(&self, head_idx: usize) -> usize {
        // headers are nodes[1..=num_cols] in the same order we created them.
        // head_idx - 1 is the column number.
        head_idx - 1
    }

    fn cover(&mut self, col_head: usize) {
        let c = self.col_index_from_head(col_head);

        // If primary, unlink from root horizontal list.
        if self.col_is_primary[c] {
            let l = self.nodes[col_head].left;
            let r = self.nodes[col_head].right;
            self.nodes[l].right = r;
            self.nodes[r].left = l;
        }

        // For each row node in this column...
        let mut i = self.nodes[col_head].down;
        while i != col_head {
            // For each node in that row (excluding i), remove it from its column vertically.
            let mut j = self.nodes[i].right;
            while j != i {
                let j_col_head = self.nodes[j].col;
                let j_c = self.col_index_from_head(j_col_head);

                let up = self.nodes[j].up;
                let down = self.nodes[j].down;
                self.nodes[up].down = down;
                self.nodes[down].up = up;

                self.col_size[j_c] -= 1;

                j = self.nodes[j].right;
            }
            i = self.nodes[i].down;
        }
    }

    fn uncover(&mut self, col_head: usize) {
        let c = self.col_index_from_head(col_head);

        // Restore rows in reverse order.
        let mut i = self.nodes[col_head].up;
        while i != col_head {
            let mut j = self.nodes[i].left;
            while j != i {
                let j_col_head = self.nodes[j].col;
                let j_c = self.col_index_from_head(j_col_head);

                let up = self.nodes[j].up;
                let down = self.nodes[j].down;
                self.nodes[up].down = j;
                self.nodes[down].up = j;

                self.col_size[j_c] += 1;

                j = self.nodes[j].left;
            }
            i = self.nodes[i].up;
        }

        // If primary, relink into root list.
        if self.col_is_primary[c] {
            let l = self.nodes[col_head].left;
            let r = self.nodes[col_head].right;
            self.nodes[l].right = col_head;
            self.nodes[r].left = col_head;
        }
    }

    fn search(&mut self, partial: &mut Vec<usize>) -> bool {
        let col = match self.choose_column() {
            None => return true, // all primary columns covered => success
            Some(c) => c,
        };

        let c_idx = self.col_index_from_head(col);
        if self.col_size[c_idx] == 0 {
            return false;
        }

        self.cover(col);

        // iterate each row that covers `col`
        let mut r = self.nodes[col].down;
        while r != col {
            partial.push(self.nodes[r].row);

            // cover other columns in the chosen row (including secondary ones)
            let mut j = self.nodes[r].right;
            while j != r {
                let j_col = self.nodes[j].col;
                self.cover(j_col);
                j = self.nodes[j].right;
            }

            if self.search(partial) {
                return true;
            }

            // backtrack: uncover in reverse order
            let mut j2 = self.nodes[r].left;
            while j2 != r {
                let j2_col = self.nodes[j2].col;
                self.uncover(j2_col);
                j2 = self.nodes[j2].left;
            }

            partial.pop();
            r = self.nodes[r].down;
        }

        self.uncover(col);
        false
    }
}

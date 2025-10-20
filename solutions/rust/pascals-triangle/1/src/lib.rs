pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return Self { rows: vec![] };
        }

        let mut rows = Vec::with_capacity(row_count as usize);
        rows.push(vec![1]);

        for i in 1..(row_count as usize) {
            let mut curr = Vec::with_capacity(i + 1);

            for j in 0..=i {
                let mut sum = 0;

                // Left parent
                if j > 0 {
                    sum += rows[i - 1][j - 1];
                }

                // Right parent
                if j < rows[i - 1].len() {
                    sum += rows[i - 1][j];
                }

                curr.push(sum);
            }

            rows.push(curr);
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

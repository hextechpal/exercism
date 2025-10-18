const DIRECTIONS: [(isize, isize); 8] = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut ans = Vec::new();

    if garden.is_empty() {
        return ans;
    }

    let matrix: Vec<&[u8]> = garden.iter().map(|s| s.as_bytes()).collect();

    let m = matrix.len();
    let n = matrix[0].len();

    for r in 0..m {
        let mut curr = String::with_capacity(n);
        for c in 0..n {
            let ch = matrix[r][c];
            if ch == b'*' {
                curr.push('*');
                continue;
            }

            let count = DIRECTIONS
                .iter()
                .filter(|&&(dx, dy)| {
                    let nr = r as isize + dx;
                    let nc = c as isize + dy;
                    nr >= 0
                        && nr < m as isize
                        && nc >= 0
                        && nc < n as isize
                        && matrix[nr as usize][nc as usize] == b'*'
                })
                .count();

            if count == 0 {
                curr.push(' ');
            } else {
                curr.push(std::char::from_digit(count as u32, 10).unwrap());
            }
        }
        ans.push(curr);
    }

    ans
}

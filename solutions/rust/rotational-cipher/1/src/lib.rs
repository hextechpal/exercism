pub fn rotate(input: &str, key: u8) -> String {
    let mut ans = String::new();
    for b in input.as_bytes() {
        match *b {
            b'a'..=b'z' => ans.push((((*b + key - b'a') % 26) + b'a') as char),
            b'A'..=b'Z' => ans.push((((*b + key - b'A') % 26) + b'A') as char),
            _ => ans.push(*b as char),
        }
    }
    ans
}

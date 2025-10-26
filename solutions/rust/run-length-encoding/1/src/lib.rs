pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut prev = '-';
    let mut count = 0;
    let mut ans = String::new();
    for curr in source.chars() {
        if !curr.is_alphanumeric() && !curr.is_whitespace() {
            continue;
        }

        if prev == '-' {
            prev = curr;
            count += 1;
            continue;
        }

        if prev != curr {
            if count == 1 {
                ans.push(prev);
            } else {
                ans.push_str(&format!("{count}{prev}"));
            }
            prev = curr;
            count = 1;
            continue;
        }

        count += 1
    }

    if count == 1 {
        ans.push(prev);
    } else {
        ans.push_str(&format!("{count}{prev}").to_string());
    }

    ans
}

pub fn decode(source: &str) -> String {
    let mut ans = String::new();
    let mut count = 0;
    let mut window = 0;
    for c in source.chars() {
        if c.is_ascii_alphabetic() || c.is_whitespace() {
            if count == 0 {
                count = 1;
            }
            for _ in 0..count {
                ans.push(c);
            }
            window = 0;
            count = 0;
            continue;
        }

        if c.is_numeric() {
            count = count * window * 10 + c.to_digit(10).unwrap();
            window += 1;
        }
    }
    ans
}

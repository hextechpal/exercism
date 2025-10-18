/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut i = 0;
    let mut sum = 0;
    for c in code.chars().rev() {
        if c == ' ' {
            continue;
        }

        if !c.is_digit(10) {
            return false;
        }

        let mut num = (c as u32) - ('0' as u32);

        if i % 2 != 0 {
            num = num * 2;
            if num > 9 {
                num = num - 9;
            }
        }
        
        sum += num;
        i += 1;
    }

    i > 1 && sum % 10 == 0
}

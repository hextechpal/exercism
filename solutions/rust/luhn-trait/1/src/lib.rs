pub trait Luhn: ToString {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let s = self.to_string();
        let mut i = 0;
        let mut sum = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                continue;
            }
            if !c.is_ascii_digit() {
                return false;
            }
            let mut num = (c as u32) - ('0' as u32);
            if i % 2 != 0 {
                num *= 2;
                if num > 9 {
                    num -= 9;
                }
            }
            sum += num;
            i += 1;
        }
        i > 1 && sum % 10 == 0
    }
}

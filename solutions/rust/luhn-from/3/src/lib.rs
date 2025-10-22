pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut i = 0;
        let mut sum = 0;
        for c in self.code.chars().rev() {
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

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl From<&str> for Luhn {
    fn from(input: &str) -> Self {
        Self {
            code: String::from(input),
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Self {
            code: input.clone(),
        }
    }
}

impl From<u8> for Luhn {
    fn from(input: u8) -> Self {
        Self {
            code: format!("{input}"),
        }
    }
}

impl From<u16> for Luhn {
    fn from(input: u16) -> Self {
        Self {
            code: format!("{input}"),
        }
    }
}

impl From<u32> for Luhn {
    fn from(input: u32) -> Self {
        Self {
            code: format!("{input}"),
        }
    }
}

impl From<u64> for Luhn {
    fn from(input: u64) -> Self {
        Self {
            code: format!("{input}"),
        }
    }
}

impl From<usize> for Luhn {
    fn from(input: usize) -> Self {
        Self {
            code: format!("{input}"),
        }
    }
}

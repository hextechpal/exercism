use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,

    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(a: &str, b: &str) -> Self {
        Self {
            from: a.to_string(),
            to: b.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();
        self
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        match self.attrs.get(key) {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}



impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
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

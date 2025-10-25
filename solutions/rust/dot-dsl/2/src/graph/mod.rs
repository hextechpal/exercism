use std::collections::HashMap;

use crate::graph::graph_items::{edge::Edge, node::Node};

pub mod graph_items;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes = nodes.iter().map(|n| n.clone()).collect();
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges = edges.iter().map(|n| n.clone()).collect();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();
        self
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
}

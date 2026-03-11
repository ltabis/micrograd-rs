use petgraph::{Graph, graph::NodeIndex};
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

pub type MyGraph = Graph<String, String>;

#[derive(Debug, Clone)]
pub enum Operator {
    Leaf,
    Add,
    Mul,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Leaf => "",
                Operator::Add => "+",
                Operator::Mul => "*",
            }
        )
    }
}

// FIXME: should encapsulate any scalar.
#[derive(Debug, Clone)]
pub struct Value {
    pub data: f64,
    pub label: Option<String>,
    children: Vec<Self>,
    operator: Operator,
}

impl Value {
    pub fn new(data: f64) -> Self {
        Self {
            data: data,
            label: None,
            children: vec![],
            operator: Operator::Leaf,
        }
    }

    pub fn new_with_label(data: f64, label: &str) -> Self {
        Self {
            data: data,
            label: Some(label.to_string()),
            children: vec![],
            operator: Operator::Leaf,
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data + rhs.data,
            label: None,
            children: vec![self.clone(), rhs.clone()],
            operator: Operator::Add,
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data * rhs.data,
            label: None,
            children: vec![self.clone(), rhs.clone()],
            operator: Operator::Mul,
        }
    }
}

impl Value {
    pub fn draw(&self) -> MyGraph {
        let graph = Graph::new();

        self.edge(graph, None)
    }

    fn edge(&self, mut graph: MyGraph, parent: Option<NodeIndex>) -> MyGraph {
        let node_data = graph.add_node(match &self.label {
            Some(label) => format!("{label} | data {:.4}", self.data),
            None => format!("data {:.4}", self.data),
        });

        match &self.operator {
            Operator::Leaf => {}
            operator => {
                let node_operator = graph.add_node(operator.to_string());

                graph.extend_with_edges(&[(node_operator, node_data)]);

                for child in &self.children {
                    graph = child.edge(graph, Some(node_operator));
                }
            }
        }

        if let Some(parent_index) = parent {
            graph.extend_with_edges(&[(node_data, parent_index)]);
        }

        graph
    }
}

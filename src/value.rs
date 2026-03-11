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
    children: Vec<Self>,
    operator: Operator,
}

impl Value {
    pub fn new(value: f64) -> Self {
        Self {
            data: value,
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
        let node_data = graph.add_node(format!("data {:.4}", self.data));

        match &self.operator {
            Operator::Leaf => {}
            operator => {
                let node_operator = graph.add_node(operator.to_string());

                graph.extend_with_edges(&[(node_data, node_operator)]);

                for child in &self.children {
                    graph = child.edge(graph, Some(node_operator));
                }
            }
        }

        if let Some(parent_index) = parent {
            graph.extend_with_edges(&[(parent_index, node_data)]);
        }

        graph
    }
}

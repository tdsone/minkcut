use petgraph::algo;
use petgraph::graph::{Graph, UnGraph};
use petgraph::prelude::*;

use petgraph::dot::{Config, Dot};
use std::fs::File;
use std::io::Write;

struct MinKCut {
    graph: UnGraph<(), f64>,
}

impl MinKCut {
    fn new() -> Self {
        MinKCut {
            graph: UnGraph::new_undirected(),
        }
    }

    fn add_node(&mut self) -> NodeIndex {
        self.graph.add_node(())
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: f64) {
        self.graph.add_edge(from, to, weight);
    }

    fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        if let Some(edge) = self.graph.find_edge(from, to) {
            self.graph.remove_edge(edge);
        }
    }

    fn export_dot(&self, filename: &str) -> std::io::Result<()> {
        let dot = format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        );
        let mut file = File::create(filename)?;
        file.write_all(dot.as_bytes())?;
        Ok(())
    }
}

fn approx_saran_vazirani(graph: &MinKCut, k: usize) {
    // TODO: Implement Saran-Vazirani approximation algorithm
    println!("Implementing Saran-Vazirani approximation for k = {}", k);
}

fn main() {
    let mut min_k_cut = MinKCut::new();

    // Create sample graph
    let n1 = min_k_cut.add_node();
    let n2 = min_k_cut.add_node();
    let n3 = min_k_cut.add_node();

    min_k_cut.add_edge(n1, n2, 2.0);
    min_k_cut.add_edge(n2, n3, 3.0);
    min_k_cut.add_edge(n3, n1, 1.0);

    // Export graph
    min_k_cut.export_dot("graph.dot").unwrap();
}

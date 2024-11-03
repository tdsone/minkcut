use core::num;

use petgraph::{
    data::Build,
    dot::Dot,
    graph::EdgeIndex,
    graph::{DiGraph, UnGraph},
};
use rand::distributions::{Distribution, Uniform};

// Creates an initial residual network for an undirected graph
fn init_residual_network(g: &UnGraph<String, u32>) -> DiGraph<String, u32> {
    let mut r = DiGraph::<String, u32>::new();

    let node_indices = g.node_indices().collect::<Vec<_>>();

    // Add all nodes from g to r
    for node in node_indices {
        let sequence = g[node].clone();
        r.add_node(sequence);
    }

    // Extract edges with positive capacities
    let edges = g.edge_indices().collect::<Vec<_>>();

    for edge in edges {
        let (u, v) = g.edge_endpoints(edge).unwrap();

        if *g.edge_weight(edge).unwrap() > 0 && u != v {
            r.add_edge(u, v, *g.edge_weight(edge).unwrap());
            r.add_edge(v, u, *g.edge_weight(edge).unwrap());
        }
    }

    r
}

fn random_dna(length: u8) -> String {
    let mut rng = rand::thread_rng();
    let rand_distr = Uniform::from(0..4);

    let nucleotides: [&str; 4] = ["A", "C", "G", "T"];

    let mut sequence = String::new();
    for _ in 0..length {
        let nt = nucleotides[rand_distr.sample(&mut rng)];
        sequence.push_str(nt);
    }

    sequence
}

fn build_graph(num_nodes: i32) -> UnGraph<String, u32> {
    let num_nodes = 10;
    let mut g = UnGraph::<String, u32>::new_undirected();

    let mut rng = rand::thread_rng();
    let rand_distr = Uniform::from(0..100);

    for _ in 0..num_nodes {
        let seq = random_dna(10);
        g.add_node(seq);
    }

    let indexes = g.node_indices().collect::<Vec<_>>();
    for i in 0..num_nodes {
        for j in i + 1..num_nodes {
            g.add_edge(indexes[i], indexes[j], rand_distr.sample(&mut rng));
        }
    }

    g
}

fn main() {
    // Create dummy fully connected undirected graph with positive edge weights

    let g = build_graph(10);

    println!("{}", Dot::new(&g));

    let r = init_residual_network(&g);

    println!("{}", Dot::new(&r));
}

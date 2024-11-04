use std::{collections::VecDeque, u32};

use petgraph::{
    dot::Dot,
    graphmap::{DiGraphMap, UnGraphMap},
};
use rand::distributions::{Distribution, Uniform};

fn build_gomory_hu_tree() {
    unimplemented!();
}

struct Partition<N, E> {
    source: DiGraphMap<N, E>,
    sink: DiGraphMap<N, E>,
}
struct MinCut<N, E> {
    flow_value: u32,
    partition: Partition<N, E>,
}

fn dinitz_bfs(
    residual: &DiGraphMap<u32, EdgeData>,
    source: u32,
    sink: u32,
) -> Option<std::collections::HashMap<u32, u32>> {
    let mut level_graph = std::collections::HashMap::new();
    let mut queue = VecDeque::new();

    level_graph.insert(source, 0);
    queue.push_back(source);

    while let Some(u) = queue.pop_front() {
        let u_level = level_graph[&u];

        for v in residual.neighbors(u) {
            let edge_data = residual.edge_weight(u, v).unwrap();
            if edge_data.capacity > edge_data.flow && !level_graph.contains_key(&v) {
                level_graph.insert(v, u_level + 1);
                queue.push_back(v);
            }
        }
    }

    if level_graph.contains_key(&sink) {
        Some(level_graph)
    } else {
        None
    }
}

fn dinitz_dfs(
    residual: &mut DiGraphMap<u32, EdgeData>,
    source: u32,
    sink: u32,
    level_graph: &std::collections::HashMap<u32, u32>,
) -> u32 {
    fn dfs(
        residual: &mut DiGraphMap<u32, EdgeData>,
        u: u32,
        sink: u32,
        flow: u32,
        level_graph: &std::collections::HashMap<u32, u32>,
        visited: &mut std::collections::HashSet<u32>,
    ) -> u32 {
        if u == sink {
            return flow;
        }

        visited.insert(u);

        // Collect neighbors and their data upfront to avoid borrow issues
        let neighbors: Vec<(u32, u32)> = residual
            .neighbors(u)
            .filter_map(|v| {
                if visited.contains(&v) {
                    return None;
                }

                if let (Some(&u_level), Some(&v_level)) = (level_graph.get(&u), level_graph.get(&v))
                {
                    if v_level == u_level + 1 {
                        let edge_data = residual.edge_weight(u, v).unwrap();
                        let available_flow = edge_data.capacity - edge_data.flow;
                        if available_flow > 0 {
                            return Some((v, available_flow));
                        }
                    }
                }
                None
            })
            .collect();

        // Now process each valid neighbor
        for (v, available_flow) in neighbors {
            let bottleneck = dfs(
                residual,
                v,
                sink,
                flow.min(available_flow),
                level_graph,
                visited,
            );

            if bottleneck > 0 {
                // Update residual network
                let edge_data = residual.edge_weight_mut(u, v).unwrap();
                edge_data.flow += bottleneck;
                let edge_data = residual.edge_weight_mut(v, u).unwrap();
                edge_data.flow -= bottleneck;
                return bottleneck;
            }
        }
        0
    }

    let mut visited = std::collections::HashSet::new();
    dfs(residual, source, sink, u32::MAX, level_graph, &mut visited)
}

// Finds the min_cut for a flow network g using Dinitz Method in O(|E||V|^2)
fn find_min_cut(
    g: &UnGraphMap<u32, u32>,
    source: u32,
    sink: u32,
    mut residual: DiGraphMap<u32, EdgeData>,
) -> MinCut<u32, u32> {
    if !g.contains_node(source) {
        panic!("Source not in graph.")
    }
    if !g.contains_node(sink) {
        panic!("Sink not in graph.");
    }
    if source == sink {
        panic!("Source is equal to sink. There is no min cut.")
    }

    // Compute max flow using Dinitz algorithm
    let mut flow_value = 0;
    loop {
        let level_graph = match dinitz_bfs(&residual, source, sink) {
            Some(lg) => lg,
            None => break,
        };

        let this_flow = dinitz_dfs(&mut residual, source, sink, &level_graph);
        if this_flow == 0 {
            break;
        }
        flow_value += this_flow;
    }

    // Find reachable nodes from source in residual graph
    let reachable = dinitz_bfs(&residual, source, sink).unwrap_or_default();

    // Build source and sink partitions based on reachability
    let mut source_partition = DiGraphMap::new();
    let mut sink_partition = DiGraphMap::new();

    // First add all nodes to their respective partitions
    for node in g.nodes() {
        if reachable.contains_key(&node) {
            source_partition.add_node(node);
        } else {
            sink_partition.add_node(node);
        }
    }

    // Optionally: Add edges between nodes in each partition
    for (u, v, weight) in g.all_edges() {
        if reachable.contains_key(&u) && reachable.contains_key(&v) {
            // Both nodes are in source partition
            source_partition.add_edge(u, v, *weight);
        } else if !reachable.contains_key(&u) && !reachable.contains_key(&v) {
            // Both nodes are in sink partition
            sink_partition.add_edge(u, v, *weight);
        }
        // Edges between partitions are not included as they form the cut
    }

    MinCut {
        flow_value,
        partition: Partition {
            source: source_partition,
            sink: sink_partition,
        },
    }
}

struct EdgeData {
    capacity: u32,
    flow: u32,
}

// Creates an initial residual network for an undirected graph
fn init_residual_network(g: &UnGraphMap<u32, u32>) -> DiGraphMap<u32, EdgeData> {
    let mut r = DiGraphMap::<u32, EdgeData>::new();

    let nodes = g.nodes();

    // Add all nodes from g to r
    for node in nodes {
        r.add_node(node);
    }

    // Extract edges with positive capacities
    let edges = g.all_edges();

    for (u, v, weight) in edges {
        if *weight > 0 && u != v {
            r.add_edge(
                u,
                v,
                EdgeData {
                    capacity: *weight,
                    flow: 0,
                },
            );
            r.add_edge(
                v,
                u,
                EdgeData {
                    capacity: *weight,
                    flow: 0,
                },
            );
        }
    }

    r
}

fn build_graph(num_nodes: u32) -> UnGraphMap<u32, u32> {
    let mut g = UnGraphMap::<u32, u32>::new();

    let mut rng = rand::thread_rng();
    let rand_distr = Uniform::from(0..100);

    for i in 0..num_nodes {
        g.add_node(i);
    }

    for i in 0..num_nodes {
        for j in i + 1..num_nodes {
            g.add_edge(i, j, rand_distr.sample(&mut rng));
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

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

fn dinitz_bfs(residual: &DiGraphMap<u32, EdgeData>, source: u32, sink: u32) -> Option<Vec<u32>> {
    let mut parents = std::collections::HashMap::new();
    let mut vertex_dist = std::collections::HashMap::new();
    let mut queue = VecDeque::new();

    vertex_dist.insert(source, 0);
    queue.push_back((source, 0));

    while let Some((u, dist)) = queue.pop_front() {
        if u == sink {
            break;
        }
        for v in residual.neighbors(u) {
            let edge_data = residual.edge_weight(u, v).unwrap();
            if edge_data.capacity > edge_data.flow {
                if let Some(&v_dist) = vertex_dist.get(&v) {
                    if v_dist == dist + 1 {
                        parents.entry(v).or_insert_with(Vec::new).push(u);
                    }
                } else {
                    parents.insert(v, vec![u]);
                    vertex_dist.insert(v, dist + 1);
                    queue.push_back((v, dist + 1));
                }
            }
        }
    }

    if parents.contains_key(&sink) {
        Some(parents.into_iter().map(|(k, _)| k).collect())
    } else {
        None
    }
}

fn dinitz_dfs(
    residual: &mut DiGraphMap<u32, EdgeData>,
    source: u32,
    sink: u32,
    parents: &std::collections::HashMap<u32, Vec<u32>>,
) -> u32 {
    let mut total_flow = 0;
    let mut path = vec![sink];
    let mut u = sink;

    while !path.is_empty() {
        if let Some(v_list) = parents.get(&u) {
            if !v_list.is_empty() {
                let v = v_list[0];
                path.push(v);
            } else {
                path.pop();
                if path.is_empty() {
                    break;
                }
                let v = *path.last().unwrap();
                if let Some(v_list) = parents.get(&v) {
                    if !v_list.is_empty() {
                        let mut v_list = v_list.clone();
                        v_list.remove(0);
                    }
                }
            }

            if let Some(&v) = path.last() {
                if v == source {
                    let mut flow = u32::MAX;
                    for window in path.windows(2) {
                        if let [u, v] = window {
                            let edge_data = residual.edge_weight(*u, *v).unwrap();
                            flow = flow.min(edge_data.capacity - edge_data.flow);
                        }
                    }

                    for window in path.windows(2).rev() {
                        if let [u, v] = window {
                            let edge_data = residual.edge_weight_mut(*v, *u).unwrap();
                            edge_data.flow += flow;
                            let edge_data = residual.edge_weight_mut(*u, *v).unwrap();
                            edge_data.flow -= flow;
                        }
                    }

                    total_flow += flow;
                    path.pop();
                }
            }
        }
        u = *path.last().unwrap_or(&sink);
    }

    total_flow
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

    let mut flow_value = 0;
    loop {
        let parents = match dinitz_bfs(&residual, source, sink) {
            Some(p) => p,
            None => break,
        };

        let this_flow = dinitz_dfs(&mut residual, source, sink, &parents);
        if this_flow == 0 {
            break;
        }
        flow_value += this_flow;
    }

    let mut source_partition = DiGraphMap::new();
    let mut sink_partition = DiGraphMap::new();

    for node in g.nodes() {
        if dinitz_bfs(&residual, source, node).is_some() {
            source_partition.add_node(node);
        } else {
            sink_partition.add_node(node);
        }
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

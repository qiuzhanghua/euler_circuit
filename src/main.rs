use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};
use std::collections::HashSet;

fn main() {
    let graph = init();
    println!("{:?}", graph);
    let paths = path_sum(&graph);
    println!("{}", paths.len());
    let mut max = 0;
    for p in paths {
        let mut path = 0;
        let mut last = p.len();
        if last <= 1 {
            continue;
        }
        last -= 1;
        while p[last] != 0 {
            last -= 1;
        }
        if last <= 1 {
            continue;
        }
        for x in 0..last {
            let edge = graph.find_edge(NodeIndex::new(p[x]), NodeIndex::new(p[x + 1]));
            if let Some(e) = edge {
                if let Some(&weight) = graph.edge_weight(e) {
                    path += weight;
                }
            }
        }
        if path > max {
            max = path;
        }
        //        println!("p = {:?}, path = {}", p, path);
    }
    println!("max path = {}", max);
}

fn path_sum(graph: &Graph<char, i32, Undirected, u32>) -> Vec<Vec<usize>> {
    fn dfs(
        graph: &Graph<char, i32, Undirected, u32>,
        start_node: usize,
        visited_edge: &mut HashSet<usize>,
        curr_path: &mut Vec<usize>,
        ans: &mut Vec<Vec<usize>>,
    ) {
        // let curr_edges: Vec<usize> = vec![];

        let start_edges = graph.edges(NodeIndex::new(start_node));
        let edge_ids: HashSet<usize> = start_edges.map(|e| e.id().index()).collect();
        if visited_edge.is_superset(&edge_ids) {
            let mut v = curr_path.clone();
            v.push(start_node);
            ans.push(v.to_vec());
            return;
        }
        let mut set: HashSet<usize> = HashSet::<usize>::new();
        for x in edge_ids {
            if !visited_edge.contains(&x) {
                set.insert(x);
            }
        }
        curr_path.push(start_node);
        for &e in &set {
            let next = next_node(&graph, e, start_node);
            visited_edge.insert(e);
            dfs(&graph, next, visited_edge, curr_path, ans);
            visited_edge.remove(&e);
        }
        curr_path.pop();
    }
    let mut ans = vec![];
    let mut visited_edge = HashSet::<usize>::new();
    let mut curr_path = vec![];

    dfs(&graph, 0, &mut visited_edge, &mut curr_path, &mut ans);

    ans
}

fn next_node(graph: &Graph<char, i32, Undirected, u32>, edge_id: usize, curr: usize) -> usize {
    match graph.edge_endpoints(EdgeIndex::new(edge_id)) {
        None => usize::MAX,
        Some((x1, x2)) => {
            if x1.index() == curr {
                x2.index()
            } else {
                x1.index()
            }
        }
    }
}

fn init() -> Graph<char, i32, Undirected, u32> {
    let mut graph = Graph::<char, i32, Undirected>::new_undirected();
    let vertx_a = graph.add_node('A');
    let vertx_b = graph.add_node('B');
    let vertx_c = graph.add_node('C');
    let vertx_d = graph.add_node('D');
    let vertx_e = graph.add_node('E');
    let vertx_f = graph.add_node('F');
    let vertx_g = graph.add_node('G');
    let vertx_h = graph.add_node('H');

    graph.extend_with_edges(&[
        (vertx_a, vertx_b, 5),
        (vertx_a, vertx_d, 4),
        (vertx_a, vertx_e, 3),
        (vertx_b, vertx_c, 4),
        (vertx_b, vertx_f, 3),
        (vertx_c, vertx_d, 5),
        (vertx_c, vertx_g, 3),
        (vertx_d, vertx_h, 3),
        (vertx_e, vertx_f, 5),
        (vertx_e, vertx_h, 4),
        (vertx_f, vertx_g, 4),
        (vertx_g, vertx_h, 5),
    ]);

    graph
}

/*
fn init2() -> Graph<char, i32, Undirected, u32> {
    Graph::<char, i32, Undirected>::from_edges(&[
        (0, 1, 5),
        (0, 3, 4),
        (0, 4, 3),
        (1, 2, 4),
        (1, 5, 3),
        (2, 3, 5),
        (2, 6, 3),
        (3, 7, 3),
        (4, 5, 5),
        (4, 7, 4),
        (5, 6, 4),
        (6, 7, 5),
    ])
}

fn init3() -> Graph<char, i32, Undirected, u32> {
    Graph::<char, i32, Undirected>::from_edges(&[
        (0, 1, 1),
        (0, 1, 2),
        (0, 1, 4),
        (1, 2, 8),
        (1, 2, 16),
    ])
}

fn init4() -> Graph<char, i32, Undirected, u32> {
    Graph::<char, i32, Undirected>::from_edges(&[
        (0, 1, 1),
        (1, 2, 2),
        (1, 3, 4),
        (2, 4, 8),
        (2, 5, 16),
        (5, 0, 32),
    ])
}
*/

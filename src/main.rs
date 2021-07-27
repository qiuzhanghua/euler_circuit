use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};
use std::collections::{HashSet, VecDeque};

fn main() {
    let mut graph = init3();
    println!("{:?}", graph);
    // let graph = init2();
    // println!("{:?}", graph);

    // let nbs = graph.neighbors(NodeIndex::new(5));
    // nbs.into_iter().for_each(|x| println!("{:?}", x));
    // let edges = graph.edges(NodeIndex::new(5));
    // edges.into_iter().for_each(|x| println!("{:?}", x.id()));
    // let node_map = dijkstra(&graph, NodeIndex::new(0), Some(NodeIndex::new(1)), |edge_ref| 1);
    // println!("{:?}", node_map);
    let longest = longest_path(&mut graph);
    println!("{}", longest);
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

fn longest_path(graph: &mut Graph<char, i32, Undirected, u32>) -> u32 {
    if graph.node_count() <= 2 || graph.edge_count() < 1 {
        return 0;
    }
    let start: usize = 0;

    let mut start_edges = graph.edges(NodeIndex::new(start as usize));
    for edge in start_edges {
        println!("========");
        let mut visited = Vec::<usize>::new();
        let mut stack = VecDeque::<usize>::new();
        let mut stack_of_node = VecDeque::<usize>::new();
        stack_of_node.push_back(start);

        stack_of_node.push_back(next_node(&graph, edge.id().index(), start));
        stack.push_back(edge.id().index());
        while !stack.is_empty() {
            let e = stack.pop_back().unwrap();
            let this_node = stack_of_node.pop_back().unwrap();
            // let this_node = next_node(&graph, e, start);
            if !visited.contains(&e) {
                visited.push(e);
                for x in graph.edges(NodeIndex::new(this_node)) {
                    let id = x.id().index();
                    stack.push_back(id);
                    stack_of_node.push_back(next_node(&graph, id, this_node));
                }
            } else {
                // println!("{:?}", &stack_of_node);
            }
            println!("visited: {:?}", &visited);
            println!("stack: {:?}", &stack);
            println!("node: {:?}", &stack_of_node);
        }
    }
    0
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
        (vertx_b, vertx_f, 5),
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

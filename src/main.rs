use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};

fn main() {
    let graph = init();
    println!("{:?}", graph);
    // let graph = init2();
    // println!("{:?}", graph);

    let nbs = graph.neighbors(NodeIndex::new(5));
    nbs.into_iter().for_each(|x| println!("{:?}", x));
    let edges = graph.edges(NodeIndex::new(5));
    edges.into_iter().for_each(|x| println!("{:?}", x.id()));
}

fn init() -> Graph<char, u32, Undirected, u32> {
    let mut graph = Graph::<char, u32, Undirected>::new_undirected();
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

fn init2() -> Graph<char, u32, Undirected, u32> {
    Graph::<char, u32, Undirected>::from_edges(&[
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

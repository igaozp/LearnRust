trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

struct Node;

struct Edge;

struct MyGraph;

// 实现关联类型
impl Graph for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

let graph = MyGraph;
let obj = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> uint {
    // do something
}
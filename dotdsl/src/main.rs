use dotdsl::graph::graph_items::node::Node;
use dotdsl::graph::Graph;

fn main() {
    println!("MAIN\n");
    let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
    let graph = Graph::new().with_nodes(
        &["a", "b", "c"]
            .iter()
            .zip(attributes.iter())
            .map(|(name, &attr)| Node::new(name).with_attrs(&[attr]))
            .collect::<Vec<_>>(),
    );

    let a = graph.node("a").expect("node a must be stored");

    println!("A = {:?}", a);
    println!("B = {:?}", graph.node("a").unwrap().attrs.get("foo"));
}
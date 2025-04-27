pub mod graph {
    use graph_items::node::Node;


    //use play::graph::graph_items::edge::Edge;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: String,
        pub attrs: String,
    }

    pub mod graph_items {
        pub mod edge {
            pub struct Edge {

            }
        }
        pub mod node {
            #[derive(PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub nnnnn: Vec<String>
            }
            impl Node {
                pub fn new(_s: &str) -> Self {
                    Node { name: "".to_string(), nnnnn:Vec::new() }
                }

                pub fn with_attrs(&self, _c: &[(&str, &str)]) {

                }
            }
        }
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph { 
                nodes: Vec::new(),
                edges: "".to_string(),
                attrs: "".to_string(),
             }
        }

        pub fn with_nodes(&self, _v: &Vec<Node>) -> &Graph {
            self
        }

        

    }
}



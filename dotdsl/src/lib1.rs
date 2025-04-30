pub mod graph {
    use graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Vec<(String, String)>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Vec::new()
            }
        }

        pub fn with_nodes(self, v: &[Node]) -> Self {
            //self.nodes.append(&mut v);
            self
        }

        pub fn with_edges(mut self, v: &Vec<Edge>) -> Self {
            let mut vvv = v.to_vec();
            self.edges.append(&mut vvv);
            self
        }

        pub fn with_attrs(mut self, atts: &[(&str, &str)]) -> Self{
            for (trt, val) in atts {
                self.attrs.push((trt.to_string(), val.to_string()));
            }
            self
        }
    }

    pub mod graph_items {

        pub mod edge {

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub edge: (String, String),
                pub attrs: Vec<(String, String)>
            }

            impl Edge {
                pub fn new(tr: &str, val: &str) -> Edge {
                    Edge { edge: (tr.to_string(), val.to_string()), attrs: Vec::new() }
                }

                pub fn with_attrs<'a>(&'a mut self, atts: &[(&str, &str)]) -> &'a mut Edge {
                    for (trt, val) in atts {
                        self.attrs.push((trt.to_string(), val.to_string()));
                    }
                    self
                }
            }

        }

        pub mod node {

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub node: String,
                pub attrs: Vec<(String, String)>
            }

            impl Node {
                pub fn new(n: &str) -> Self {
                    Node { node: n.to_string(), attrs: Vec::new() }
                }

                pub fn with_attrs<'a>(&'a mut self, atts: &[(&str, &str)]) -> &'a mut Node {
                    for (trt, val) in atts {
                        self.attrs.push((trt.to_string(), val.to_string()));
                    }
                    self
                }

            }
        
        }

    }

}

// Check out https://users.rust-lang.org/t/vec-mut-t-and-vec-t-mismatch-in-builder-pattern/58151/4

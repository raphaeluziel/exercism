pub mod graph {
    use std::collections::HashMap;
    use graph_items::{edge::Edge, node::Node};

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<&'a str, &'a str>
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(mut self, v: &[Node<'a>]) -> Self {
            self.nodes.append(&mut v.to_owned());
            self
        }

        pub fn with_edges(mut self, v: &[Edge<'a>]) -> Self {
            self.edges.append(&mut v.to_owned());
            self
        }

        pub fn with_attrs(mut self, atts: &[(&'a str, &'a str)]) -> Self {
            for (trt, val) in atts {
                self.attrs.insert(trt, val);
            }
            self
        }

        pub fn node(&self, n: &'a str) -> Option<&Node> {
            match self.nodes.iter().filter(|x| x.node == n).last() {
                Some(x) => Some(x),
                None => None
            }
        }

    }

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge<'a> {
                pub edge: (&'a str, &'a str),
                pub attrs: HashMap<&'a str, &'a str>
            }
    
            impl<'a> Edge<'a> {
                pub fn new(tr: &'a str, val: &'a str) -> Edge<'a> {
                    Edge { edge: (tr, val), attrs: HashMap::new() }
                }
    
                pub fn with_attrs(mut self, atts: &[(&'a str, &'a str)]) -> Edge<'a> {
                    for (trt, val) in atts {
                        self.attrs.insert(trt, val);
                    }
                    self
                }

                pub fn attr(&self, att: &'a str) -> Option<&'a str> {
                    self.attrs.get(att).map(|v| &**v)
                }
            }
    
        }
    
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node<'a> {
                pub node: &'a str,
                pub attrs: HashMap<&'a str, &'a str>
            }
    
            impl<'a> Node<'a> {
                pub fn new(n: &'a str) -> Self {
                    Node { node: n, attrs: HashMap::new() }
                }
    
                pub fn with_attrs(mut self, atts: &[(&'a str, &'a str)]) -> Self {
                    for (trt, val) in atts {
                        self.attrs.insert(trt, val);
                    }
                    self
                }

                pub fn attr(&self, att: &'a str) -> Option<&'a str> {
                    self.attrs.get(att).map(|v| &**v)
                }
    
            }
        
        }
    
    }

}

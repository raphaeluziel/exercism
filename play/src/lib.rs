pub mod graph {
    use graph_items::{edge::Edge, node::Node};

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: String
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Graph<'a> {
            Graph {
                nodes: Vec::new(), 
                edges: Vec::new(),
                attrs: "".to_string()
            }
        }

        pub fn with_nodes(&mut self, v: &Vec<Node<'a>>) -> &'a Graph {
            for nd in v {
                self.nodes.push(nd.clone());
            }
            self
        }

        pub fn with_edges(&mut self, v: &Vec<Edge<'a>>) -> &'a mut Graph {
            for ed in v {
                self.edges.push(*ed);
            }
            self
        }
    }

    pub mod graph_items {

        pub mod edge {

            #[derive(Debug, Clone, Copy)]
            pub struct Edge<'a> {
                pub edge: (&'a str, &'a str)
            }

            impl<'a> Edge<'a> {
                pub fn new(tr: &'a str, val: &'a str) -> Self {
                    Edge { edge: (tr, val) }
                }
            }

        }

        pub mod node {

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node<'a> {
                pub node: &'a str,
                pub attrs: Vec<(&'a str, &'a str)>
            }

            impl<'a> Node<'a> {
                pub fn new(n: &'a str) -> Self {
                    Node { node: n, attrs: Vec::new() }
                }

                pub fn with_attrs(&mut self, atts: &[(&'a str, &'a str)]) -> &'a mut Node {
                    for &at in atts {
                        self.attrs.push(at);
                    }
                    self
                }

            }
        
        }

    }

}



// pub struct Command {
//     program: String,
//     args: Vec<String>,
//     cwd: Option<String>,
//     // etc
// }

// impl Command {
//     pub fn new(program: String) -> Command {
//         Command {
//             program: program,
//             args: Vec::new(),
//             cwd: None,
//         }
//     }

//     /// Add an argument to pass to the program.
//     pub fn arg<'a>(&'a mut self, arg: String) -> &'a mut Command {
//         self.args.push(arg);
//         self
//     }

//     /// Add multiple arguments to pass to the program.
//     pub fn args<'a>(&'a mut self, args: &[String])
//                     -> &'a mut Command {
//         self.args.push_all(args);
//         self
//     }

//     /// Set the working directory for the child process.
//     pub fn cwd<'a>(&'a mut self, dir: String) -> &'a mut Command {
//         self.cwd = Some(dir);
//         self
//     }

//     /// Executes the command as a child process, which is returned.
//     pub fn spawn(&self) -> IoResult<Process> {
//         ...
//     }
// }



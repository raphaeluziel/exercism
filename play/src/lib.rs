pub mod graph {
    use graph_items::node::Node;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: String,
        pub attrs: String
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(), 
                edges: "".to_string(),
                attrs: "".to_string()
            }
        }

        pub fn with_nodes(&mut self, v: &Vec<Node<'a>>) {
            for nd in v {
                &self.nodes.push(nd);
            }
        }
    }

    pub mod graph_items {

        pub mod edge {

            pub struct Edge;

        }

        pub mod node {

            pub struct Node<'a> {
                node: &'a str
            }

            impl<'a> Node<'a> {
                pub fn new(n: &'a str) -> Self {
                    Node { node: n }
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



use crate::ts::{Node, Tree};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::Write;

/// A lightweight identifier for a Tree-sitter node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId {
    pub file: String,
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
}
impl NodeId {
    pub fn from_node(file: &str, node: &Node) -> Self {
        NodeId {
            file: file.to_string(),
            kind: node.kind().to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
        }
    }

    pub fn root_node() -> Self {
        NodeId {
            file: "__root__".to_string(),
            kind: "root".to_string(),
            start_byte: 0,
            end_byte: 0,
        }
    }
}
impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}:{}:{}-{}",
            self.file, self.kind, self.start_byte, self.end_byte
        )
    }
}

/// The dependency graph structure.
pub struct TypeDependencyGraph {
    pub graph: Graph<NodeId, String, Directed>,
    pub node_indices: HashMap<NodeId, NodeIndex>,
}

impl TypeDependencyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let mut node_indices = HashMap::new();

        // Create a root node to serve as the starting point of the graph. i.e, project root.
        let root_node = NodeId::root_node();
        let root_idx = graph.add_node(root_node.clone());
        node_indices.insert(root_node, root_idx);

        Self {
            graph,
            node_indices,
        }
    }

    pub fn process_tree(&mut self, file_path: &String, tree: &Tree) {
        let capture_nodes = vec!["program", "class_declaration", "method_declaration"];

        fn traverse(
            file_path: &str,
            node: Node,
            capture_nodes: &[&str],
            graph: &mut TypeDependencyGraph,
            parent: Option<&NodeId>,
        ) {
            let mut current_node_id = None;
            if capture_nodes.contains(&node.kind()) {
                let node_id = NodeId::from_node(file_path, &node);
                graph.add_node(node_id.clone());
                if let Some(parent_id) = parent {
                    graph.add_dependency(node_id.clone(), parent_id.clone());
                }
                // If this is a "program" node, add edge from it to root node
                if node.kind() == "program" {
                    let root_node = NodeId::root_node();
                    graph.add_dependency(node_id.clone(), root_node);
                }
                current_node_id = Some(node_id);
            }

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                traverse(
                    file_path,
                    child,
                    capture_nodes,
                    graph,
                    current_node_id.as_ref().or(parent),
                );
            }
        }

        let root_node = tree.root_node();
        traverse(file_path, root_node, &capture_nodes, self, None);
    }

    /// Add a node if it doesn't exist, and return its index.
    pub fn add_node(&mut self, node: NodeId) -> NodeIndex {
        if let Some(&idx) = self.node_indices.get(&node) {
            idx
        } else {
            let idx = self.graph.add_node(node.clone());
            self.node_indices.insert(node, idx);
            idx
        }
    }

    /// Add a dependency edge between two nodes with an empty label.
    pub fn add_dependency(&mut self, from: NodeId, to: NodeId) {
        let from_idx = self.add_node(from);
        let to_idx = self.add_node(to);
        self.graph.add_edge(from_idx, to_idx, String::new());
    }

    /// Export the dependency graph to a DOT file.
    pub fn export_to_dot(&self, path: &str) -> std::io::Result<()> {
        let dot = Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        let mut file = File::create(path)?;
        write!(file, "{}", dot)?;
        Ok(())
    }
}

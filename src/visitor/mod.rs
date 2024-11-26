use crate::parser::{Node, TSParsers, Tree};

pub struct TreeVisitor<'a> {
    pub parsers: &'a TSParsers,
    pub language: String,
}

impl<'a> TreeVisitor<'a> {
    pub fn new(parsers: &'a TSParsers, language: String) -> Self {
        Self { parsers, language }
    }

    pub fn get_method_nodes(
        &self,
        tree: &'a Tree,
        source_code: &'a str,
    ) -> Vec<(Node<'a>, String)> {
        let query_string = match self.language.as_str() {
            "Java" => "[(constructor_declaration ) @definition (method_declaration ) @definition]",
            "Python" => "(function_definition ) @definition",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return Vec::new(); // Return an empty vector for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return Vec::new();
            }
        };

        let root_node = tree.root_node();
        parser.query_tree(&root_node, tree, source_code, query_string)
    }

    pub fn get_method_child_nodes(
        &self,
        method_node: Node,
        tree: &'a Tree,
        source_code: &'a str,
    ) -> Vec<(Node<'a>, String)> {
        let query_string = match self.language.as_str() {
            "Java" => "(_ name: (_) @name)",
            "Python" => "(_ name: (_) @name)",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return Vec::new(); // Return an empty vector for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return Vec::new();
            }
        };

        parser.query_tree(&method_node, tree, source_code, query_string)
    }
}

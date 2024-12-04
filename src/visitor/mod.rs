use crate::parser::{Node, TSParsers, Tree};

pub struct TreeVisitor<'a> {
    pub parsers: &'a TSParsers,
    pub language: String,
}

impl<'a> TreeVisitor<'a> {
    pub fn new(parsers: &'a TSParsers, language: String) -> Self {
        Self { parsers, language }
    }

    pub fn get_class_nodes(
        &self,
        node: &'a Node,
        tree: &'a Tree,
        source_code: &'a str,
    ) -> Vec<(Node<'a>, String)> {
        let query_string = match self.language.as_str() {
            "Java" => "(class_declaration ) @definition",
            "Python" => "(class_definition ) @definition",
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

        // let root_node = tree.root_node();
        parser.query_tree(&node, tree, source_code, query_string)
    }

    pub fn get_class_name(&self, class_node: Node, tree: &'a Tree, source_code: &'a str) -> String {
        let query_string = match self.language.as_str() {
            "Java" => "(class_declaration name: (_) @name)",
            "Python" => "(class_definition name: (_) @name)",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return "".to_string(); // Return an empty string for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return "".to_string();
            }
        };

        let query_result = parser.query_tree(&class_node, &tree, source_code, query_string);
        let method_name_node = query_result.first();
        let method_name_text = match method_name_node {
            Some((name_node, _)) => name_node.utf8_text(source_code.as_bytes()).unwrap(),
            None => "unknown",
        };
        method_name_text.to_string()
    }

    pub fn get_method_nodes(
        &self,
        node: &'a Node,
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

        parser.query_tree(&node, tree, source_code, query_string)
    }

    pub fn get_method_name(
        &self,
        method_node: Node,
        tree: &'a Tree,
        source_code: &'a str,
    ) -> String {
        let query_string = match self.language.as_str() {
            "Java" => {
                "[(constructor_declaration name: (_) @name)(method_declaration name: (_) @name)]"
            }
            "Python" => "(function_definition name: (_) @name)",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return "".to_string(); // Return an empty string for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return "".to_string();
            }
        };

        let query_result = parser.query_tree(&method_node, &tree, &source_code, query_string);
        let method_name_node = query_result.first();
        let method_name_text = match method_name_node {
            Some((name_node, _)) => name_node.utf8_text(source_code.as_bytes()).unwrap(),
            None => "unknown",
        };
        method_name_text.to_string()
    }

    pub fn count_parameters(
        &self,
        method_node: Node,
        tree: &'a Tree,
        source_code: &'a str,
    ) -> usize {
        let query_string = match self.language.as_str() {
            "Java" => "(_ parameters: (_(_) @param ))",
            "Python" => "(_ parameters: (_(_) @param ))",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return 0; // Return 0 for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return 0;
            }
        };

        let query_result = parser.query_tree(&method_node, tree, source_code, query_string);
        let param_count = query_result.len();
        param_count
    }

    pub fn count_empty_lines(&self, node: Node, source_code: &str) -> usize {
        let mut empty_lines_count = 0;

        // Extract the text of the node
        if let Some(node_text) = source_code.get(node.start_byte()..node.end_byte()) {
            // Iterate through lines in the node's text
            for line in node_text.lines() {
                // Check if the line is empty or contains only whitespace
                if line.trim().is_empty() {
                    empty_lines_count += 1;
                }
            }
        }

        empty_lines_count
    }

    pub fn count_comments(&self, node: Node, tree: &'a Tree, source_code: &str) -> (usize, usize) {
        let query_string = match self.language.as_str() {
            "Java" => "[(line_comment) @comment (block_comment) @comment]",
            "Python" => "[(comment) @comment (expression_statement (string) @comment)]",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return (0, 0); // Return 0 for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return (0, 0);
            }
        };

        let query_result = parser.query_tree(&node, tree, source_code, query_string);

        let mut total_comments_count = 0;
        let mut doc_comments_count = 0;

        for (node, _) in query_result {
            total_comments_count += 1;

            // Extract the text of the comment
            if let Ok(comment_text) = node.utf8_text(source_code.as_bytes()) {
                if self.language == "Java" {
                    // Check for Java doc comments (start with /**)
                    if comment_text.starts_with("/**") {
                        doc_comments_count += 1;
                    }
                } else if self.language == "Python" {
                    // Check for Python docstrings (triple quotes)
                    if comment_text.starts_with("\"\"\"") || comment_text.starts_with("'''") {
                        doc_comments_count += 1;
                    }
                }
            }
        }

        (total_comments_count, doc_comments_count)
    }

    pub fn count_imports(&self, node: Node, tree: &'a Tree, source_code: &str) -> usize {
        let query_string = match self.language.as_str() {
            "Java" => "(import_declaration) @import",
            "Python" => "[(import_statement) @import (import_from_statement) @import]",
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return 0; // Return 0 for unsupported languages
            }
        };

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return 0;
            }
        };

        let query_result = parser.query_tree(&node, tree, source_code, query_string);
        query_result.len() // Return the count of captured import nodes
    }

    pub fn check_if_broken(&self, node: Node) -> bool {
        // NOTE: COMPUTE HEAVY FUNCTION, maybe?

        let skip_nodes = match self.language.as_str() {
            "Java" => vec![
                "class_declaration",
                "method_declaration",
                "constructor_declaration",
            ],
            "Python" => vec!["class_definition", "function_definition"],
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return false; // Return 0 for unsupported languages
            }
        };

        let mut is_broken = false;

        fn traverse(node: Node, is_broken: &mut bool, skip_nodes: &[&str]) {
            let mut cursor = node.walk();
            if node.kind() == "ERROR" || node.is_missing() {
                *is_broken = true;
                return;
            }

            for child in node.children(&mut cursor) {
                if skip_nodes.contains(&child.kind()) {
                    continue;
                }
                traverse(child, is_broken, skip_nodes);
            }
        }

        traverse(node, &mut is_broken, &skip_nodes);
        is_broken
    }
}

use crate::parser::{Node, TSParsers, Tree};

pub fn get_query_group<'a>(language: &'a str, query_name: &'a str) -> &'a str {
    const JAVA_BASE_QUERY: &str = concat!(
        "[(line_comment) @comment (block_comment) @comment]",
        "(import_declaration) @import",
        "(class_declaration) @class_definition",
        "[(constructor_declaration) @method_definition (method_declaration) @method_definition]",
    );

    const PYTHON_BASE_QUERY: &str = concat!(
        "[(comment) @comment (expression_statement (string) @comment)]",
        "[(import_statement) @import (import_from_statement) @import]",
        "(class_definition) @class_definition",
        "(function_definition ) @method_definition",
    );

    match (language, query_name) {
        ("Java", "base_query") => JAVA_BASE_QUERY,
        ("Python", "base_query") => PYTHON_BASE_QUERY,
        _ => {
            eprintln!(
                "Unsupported language or group name: {} - {}",
                language, query_name
            );
            ""
        }
    }
}

pub struct TreeVisitor<'a> {
    pub parsers: &'a TSParsers,
    pub language: String,
    pub source_code: &'a str,
}

impl<'a> TreeVisitor<'a> {
    pub fn new(parsers: &'a TSParsers, language: &String, source_code: &'a str) -> Self {
        Self {
            parsers,
            language: language.to_string(),
            source_code,
        }
    }

    pub fn perform_base_query(
        &self,
        node: &'a Node,
        tree: &'a Tree,
    ) -> (Vec<Node<'a>>, Vec<Node<'a>>, Vec<Node<'a>>, Vec<Node<'a>>) {
        let query_string = get_query_group(&self.language, "base_query");
        let mut comment_n = Vec::new();
        let mut import_n = Vec::new();
        let mut class_n = Vec::new();
        let mut method_n = Vec::new();

        let parser = match self.parsers.get_parser(&self.language) {
            Some(p) => p,
            None => {
                eprintln!("Parser not found for language: {}", self.language);
                return (comment_n, import_n, class_n, method_n);
            }
        };

        let query_result = parser.query_tree(&node, tree, self.source_code, query_string);

        for (matched_node, capture_name) in query_result {
            match capture_name.as_str() {
                "comment" => comment_n.push(matched_node),
                "import" => import_n.push(matched_node),
                "class_definition" => class_n.push(matched_node),
                "method_definition" => method_n.push(matched_node),
                _ => {}
            }
        }

        (comment_n, import_n, class_n, method_n)
    }

    pub fn get_class_name(&self, class_node: &Node) -> String {
        let class_name_node = class_node.child_by_field_name("name").unwrap();
        let class_name_text = class_name_node
            .utf8_text(self.source_code.as_bytes())
            .unwrap();
        class_name_text.to_string()
    }

    pub fn get_method_name(&self, method_node: &Node) -> String {
        let method_name_node = method_node.child_by_field_name("name").unwrap();
        let method_name_text = method_name_node
            .utf8_text(self.source_code.as_bytes())
            .unwrap();
        method_name_text.to_string()
    }

    pub fn count_parameters(&self, method_node: &Node) -> usize {
        let parameters_node = method_node.child_by_field_name("parameters").unwrap();
        let parameters_count = parameters_node.child_count();
        parameters_count
    }

    pub fn count_empty_lines(&self, node: Node) -> usize {
        let mut empty_lines_count = 0;

        // Extract the text of the node
        if let Some(node_text) = self.source_code.get(node.start_byte()..node.end_byte()) {
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

    pub fn count_comments(&self, comment_nodes: &Vec<Node>) -> (usize, usize) {
        let mut total_comments_count = 0;
        let mut doc_comments_count = 0;

        for node in comment_nodes {
            total_comments_count += 1;

            // Extract the text of the comment
            if let Ok(comment_text) = node.utf8_text(self.source_code.as_bytes()) {
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

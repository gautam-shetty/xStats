use crate::parser::{Node, TSParsers, Tree};
use crate::utils::get_file_name;
use crate::visitor::TreeVisitor;

pub fn get_node_groups() -> Vec<(&'static str, Vec<(&'static str, Vec<&'static str>)>)> {
    vec![
        (
            "Java",
            vec![
                (
                    "decision_point_nodes",
                    vec![
                        "if_statement",
                        "for_statement",
                        "while_statement",
                        "do_statement",
                        "switch_expression",
                        "switch_statement",
                        "catch_clause",
                        "conditional_expression",
                        "lambda_expression",
                        "method_reference",
                    ],
                ),
                (
                    "function_definition",
                    vec!["method_declaration", "constructor_declaration"],
                ),
            ],
        ),
        (
            "Python",
            vec![
                (
                    "decision_point_nodes",
                    vec![
                        "if_statement",
                        "for_statement",
                        "while_statement",
                        "with_statement",
                        "try_statement",
                        "except_clause",
                        "match_statement",
                        "case_clause",
                        "conditional_expression",
                        "lambda",
                    ],
                ),
                ("function_definition", vec!["function_definition"]),
            ],
        ),
    ]
}

/// Represents metrics for a piece of code, such as a method or function.
pub struct CodeMetric {
    /// The programming language of the source file.
    pub language: String,
    /// The file path of the source file.
    pub file_path: String,
    /// The name of the node (e.g., method or function name).
    pub node_name: String,
    /// The type of the node (e.g., function, method, class).
    pub node_type: String,
    /// The starting line number of the node in the source file.
    pub start_row: u32,
    /// The starting column number of the node in the source file.
    pub start_col: u32,
    /// The ending line number of the node in the source file.
    pub end_row: u32,
    /// The ending column number of the node in the source file.
    pub end_col: u32,
    /// The number of lines of code in the node.
    pub loc: u32,
    /// The cyclomatic complexity of the node.
    pub cc: u32,
    /// The number of parameters the node takes.
    pub pc: u32,
}

pub struct CodeMetrics {
    pub metrics: Vec<CodeMetric>,
}

impl CodeMetric {
    pub fn new(
        language: String,
        file_path: String,
        node_name: String,
        node_type: String,
    ) -> CodeMetric {
        CodeMetric {
            language,
            file_path,
            node_name,
            node_type,
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
            loc: 0,
            cc: 0,
            pc: 0,
        }
    }

    pub fn generate_node_metrics(&mut self, node: &Node) {
        self.load_range(node);
        self.calculate_loc(node);
        self.calculate_cc(node);
    }

    pub fn load_range(&mut self, node: &Node) {
        let start_position = node.start_position();
        let end_position = node.end_position();
        self.start_row = start_position.row as u32;
        self.start_col = start_position.column as u32;
        self.end_row = end_position.row as u32;
        self.end_col = end_position.column as u32;
    }

    pub fn calculate_loc(&mut self, node: &Node) {
        // let root_node = tree.root_node();
        let start_line = node.start_position().row;
        let end_line = node.end_position().row;
        self.loc = (end_line - start_line + 1) as u32;
    }

    fn count_decision_points(&self, node: Node) -> usize {
        let mut count = 0;

        // Get the decision points for the current language
        let node_groups = get_node_groups();
        let empty_vec = vec![];
        let decision_points = node_groups
            .iter()
            .find(|(lang, _)| *lang == self.language)
            .map(|(_, groups)| {
                groups
                    .iter()
                    .find(|(group_name, _)| *group_name == "decision_point_nodes")
                    .map(|(_, points)| points)
                    .unwrap_or(&empty_vec)
            })
            .unwrap_or(&empty_vec);

        // Check if the node is a decision point
        if decision_points.contains(&node.kind()) {
            count += 1;
        }

        // Traverse child nodes
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                count += self.count_decision_points(child);
            }
        }

        count
    }

    pub fn calculate_cc(&mut self, node: &Node) {
        self.cc = self.count_decision_points(*node) as u32 + 1;
    }
}

impl CodeMetrics {
    pub fn new() -> CodeMetrics {
        CodeMetrics {
            metrics: Vec::new(),
        }
    }

    pub fn add_metrics(&mut self, metrics: CodeMetric) {
        self.metrics.push(metrics);
    }

    pub fn generate_root_metrics(&mut self, language: String, file_path: String, tree: &Tree) {
        let root_node = tree.root_node();
        let root_type = root_node.kind();
        let mut metrics = CodeMetric::new(
            language,
            file_path.clone(),
            get_file_name(&file_path),
            root_type.to_string(),
        );
        metrics.generate_node_metrics(&root_node);
        self.add_metrics(metrics);
    }

    pub fn generate_function_metrics(
        &mut self,
        parsers: &TSParsers,
        source_code: &str,
        language: String,
        file_path: String,
        tree: &Tree,
    ) {
        let visitor = TreeVisitor::new(parsers, language.clone());
        let method_captures = visitor.get_method_nodes(tree, source_code);
        for (node, tag) in method_captures {
            let node_type = node.kind();
            let method_child_nodes = visitor.get_method_child_nodes(node, tree, source_code);
            let method_name_node = method_child_nodes.first();
            let method_name_text = match method_name_node {
                Some((name_node, _)) => name_node.utf8_text(source_code.as_bytes()).unwrap(),
                None => "unknown",
            };
            let mut metrics = CodeMetric::new(
                language.clone(),
                file_path.clone(),
                method_name_text.to_string(),
                node_type.to_string(),
            );
            metrics.generate_node_metrics(&node);
            self.add_metrics(metrics);
        }
    }
}

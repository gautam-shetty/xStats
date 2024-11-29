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
    /// The number of actual lines of code in the node.
    pub aloc: u32,
    /// The number of empty lines of code in the node.
    pub eloc: u32,
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
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
            node_name,
            node_type,
            aloc: 0,
            eloc: 0,
            cc: 0,
            pc: 0,
        }
    }

    pub fn generate_node_metrics(&mut self, node: &Node) {
        self.load_range(node);
        self.calculate_aloc(node);
    }

    fn load_range(&mut self, node: &Node) {
        let start_position = node.start_position();
        let end_position = node.end_position();
        self.start_row = start_position.row as u32;
        self.start_col = start_position.column as u32;
        self.end_row = end_position.row as u32;
        self.end_col = end_position.column as u32;
    }

    pub fn load_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    fn calculate_aloc(&mut self, node: &Node) {
        let start_line = node.start_position().row;
        let end_line = node.end_position().row;
        self.aloc = (end_line - start_line + 1) as u32;
    }

    pub fn calculate_eloc(&mut self, node: &Node, source_code: &str, visitor: &TreeVisitor) {
        self.eloc = visitor.count_empty_lines(*node, source_code) as u32;
    }

    fn count_decision_points(&self, node: Node, decision_points: &Vec<&str>) -> usize {
        let mut count = 0;

        // Check if the current node is a decision point
        if decision_points.contains(&node.kind()) {
            count += 1;
        }

        // Traverse child nodes iteratively to avoid stack overflow
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            // Add child nodes to the stack
            for i in 0..current.child_count() {
                if let Some(child) = current.child(i) {
                    if decision_points.contains(&child.kind()) {
                        count += 1;
                    }
                    stack.push(child);
                }
            }
        }

        count
    }

    pub fn calculate_cc(&mut self, node: &Node) {
        // Get the decision points for the current language
        let empty_vec = vec![];
        let node_groups = get_node_groups();
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

        self.cc = self.count_decision_points(*node, decision_points) as u32 + 1;
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

            let method_name = visitor.get_method_name(node, tree, source_code);
            let parameters_count = visitor.get_parameters_count(node, tree, source_code);

            let mut metrics = CodeMetric::new(
                language.clone(),
                file_path.clone(),
                method_name,
                node_type.to_string(),
            );
            metrics.generate_node_metrics(&node);
            metrics.load_pc(parameters_count as u32);
            metrics.calculate_eloc(&node, source_code, &visitor);
            metrics.calculate_cc(&node);

            self.add_metrics(metrics);
        }
    }
}

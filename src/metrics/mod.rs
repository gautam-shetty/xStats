use crate::parser::{Node, TSParsers, Tree};
use crate::utils::get_file_name;
use crate::visitor::TreeVisitor;

pub fn get_node_groups() -> Vec<(&'static str, Vec<(&'static str, Vec<&'static str>)>)> {
    vec![
        (
            "Java",
            vec![(
                "decision_point_nodes",
                vec![
                    "if_statement",
                    "else_clause",
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
            )],
        ),
        (
            "Python",
            vec![(
                "decision_point_nodes",
                vec![
                    "if_statement",
                    "elif_clause",
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
            )],
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
    /// Indicates whether the node is broken or has missing elements (e.g., syntax error).
    pub is_broken: bool,
    /// The number of actual lines of code in the node.
    pub aloc: u32,
    /// The number of empty lines of code in the node.
    pub eloc: u32,
    /// The number of comment lines of code in the node.
    pub cloc: u32,
    /// The number of document comment lines of code in the node.
    pub dcloc: u32,
    /// Number of imports in the node.
    pub noi: u32,
    /// Number of classes in the node.
    pub noc: u32,
    /// Number of methods in the node.
    pub nom: u32,
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
        language: &String,
        file_path: &String,
        node_name: String,
        node_type: String,
    ) -> CodeMetric {
        CodeMetric {
            language: language.to_string(),
            file_path: file_path.to_string(),
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
            is_broken: false,
            node_name,
            node_type,
            aloc: 0,
            eloc: 0,
            cloc: 0,
            dcloc: 0,
            noi: 0,
            noc: 0,
            nom: 0,
            cc: 0,
            pc: 0,
        }
    }

    pub fn generate_node_metrics(&mut self, node: &Node, visitor: &TreeVisitor) {
        self.load_range(node);
        self.calculate_aloc(node);
        self.check_broken(node, visitor);
    }

    fn load_range(&mut self, node: &Node) {
        let start_position = node.start_position();
        let end_position = node.end_position();
        let offset = 1;
        self.start_row = (start_position.row + offset) as u32;
        self.start_col = (start_position.column + offset) as u32;
        self.end_row = (end_position.row + offset) as u32;
        self.end_col = (end_position.column + offset) as u32;
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

    pub fn calculate_cloc_dcloc(
        &mut self,
        node: &Node,
        tree: &Tree,
        source_code: &str,
        visitor: &TreeVisitor,
    ) {
        let (cloc, dcloc) = visitor.count_comments(*node, tree, source_code);
        self.cloc = cloc as u32;
        self.dcloc = dcloc as u32;
    }

    pub fn calculate_noi(
        &mut self,
        node: &Node,
        tree: &Tree,
        source_code: &str,
        visitor: &TreeVisitor,
    ) {
        self.noi = visitor.count_imports(*node, tree, source_code) as u32;
    }

    pub fn calculate_noc(
        &mut self,
        node: &Node,
        tree: &Tree,
        source_code: &str,
        visitor: &TreeVisitor,
    ) {
        self.noc = 0;
        let class_captures = visitor.get_class_nodes(node, tree, source_code);
        self.noc = class_captures.len() as u32;
    }

    pub fn calculate_nom(
        &mut self,
        node: &Node,
        tree: &Tree,
        source_code: &str,
        visitor: &TreeVisitor,
    ) {
        self.nom = 0;
        let method_captures = visitor.get_method_nodes(node, tree, source_code);
        self.nom = method_captures.len() as u32;
    }

    fn check_broken(&mut self, node: &Node, visitor: &TreeVisitor) {
        self.is_broken = visitor.check_if_broken(*node);
    }

    fn count_decision_points(&self, node: Node, decision_points: &Vec<&str>) -> usize {
        let mut count = 0;

        let skip_nodes = match self.language.as_str() {
            "Java" => vec![
                "class_declaration",
                "method_declaration",
                "constructor_declaration",
            ],
            "Python" => vec!["class_definition", "function_definition"],
            _ => {
                eprintln!("Unsupported language: {}", self.language);
                return 0; // Return 0 for unsupported languages
            }
        };

        // Check if the child node is a decision point
        if decision_points.contains(&node.kind()) {
            count += 1;
        }

        // Traverse child nodes to count decision points
        if !skip_nodes.contains(&node.kind()) {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    count += self.count_decision_points(child, decision_points);
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

    fn add_metric(&mut self, code_metric: CodeMetric) {
        self.metrics.push(code_metric);
    }

    pub fn generate_root_metrics(
        &mut self,
        parsers: &TSParsers,
        source_code: &str,
        language: &String,
        file_path: &String,
        tree: &Tree,
    ) {
        let visitor = TreeVisitor::new(parsers, &language);

        let root_node = tree.root_node();
        let root_type = root_node.kind();
        let mut metric = CodeMetric::new(
            language,
            &file_path,
            get_file_name(&file_path),
            root_type.to_string(),
        );
        metric.generate_node_metrics(&root_node, &visitor);
        metric.calculate_eloc(&root_node, source_code, &visitor);
        metric.calculate_cloc_dcloc(&root_node, tree, source_code, &visitor);
        metric.calculate_noi(&root_node, tree, source_code, &visitor);
        metric.calculate_cc(&root_node);

        let class_nodes = visitor.get_class_nodes(&root_node, tree, source_code);
        metric.noc = class_nodes.len() as u32;

        let method_nodes = visitor.get_method_nodes(&root_node, tree, source_code);
        metric.nom = method_nodes.len() as u32;

        self.add_metric(metric);

        self.generate_class_metrics(
            &parsers,
            &source_code,
            language.to_string(),
            file_path.to_string(),
            &tree,
            &class_nodes,
            &visitor,
        );
        self.generate_function_metrics(
            &parsers,
            &source_code,
            language.to_string(),
            file_path.to_string(),
            &tree,
            &method_nodes,
            &visitor,
        );
    }

    pub fn generate_class_metrics(
        &mut self,
        parsers: &TSParsers,
        source_code: &str,
        language: String,
        file_path: String,
        tree: &Tree,
        class_captures: &Vec<(Node, String)>,
        visitor: &TreeVisitor,
    ) {
        for (node, tag) in class_captures {
            let node_type = node.kind();

            let class_name = visitor.get_class_name(*node, tree, source_code);

            let mut metric =
                CodeMetric::new(&language, &file_path, class_name, node_type.to_string());
            metric.generate_node_metrics(&node, visitor);
            metric.calculate_eloc(node, source_code, visitor);
            metric.calculate_cloc_dcloc(node, tree, source_code, visitor);
            metric.calculate_noi(node, tree, source_code, visitor);
            metric.calculate_noc(node, tree, source_code, visitor);
            metric.noc -= 1; // Exclude the class itself
            metric.calculate_nom(node, tree, source_code, visitor);
            metric.calculate_cc(node);

            self.add_metric(metric);
        }
    }

    pub fn generate_function_metrics(
        &mut self,
        parsers: &TSParsers,
        source_code: &str,
        language: String,
        file_path: String,
        tree: &Tree,
        method_captures: &Vec<(Node, String)>,
        visitor: &TreeVisitor,
    ) {
        for (node, tag) in method_captures {
            let node_type = node.kind();

            let method_name = visitor.get_method_name(*node, tree, source_code);
            let parameters_count = visitor.count_parameters(*node, tree, source_code);

            let mut metric =
                CodeMetric::new(&language, &file_path, method_name, node_type.to_string());
            metric.generate_node_metrics(&node, visitor);
            metric.load_pc(parameters_count as u32);
            metric.calculate_eloc(node, source_code, visitor);
            metric.calculate_cloc_dcloc(node, tree, source_code, visitor);
            metric.calculate_noi(node, tree, source_code, visitor);
            metric.calculate_noc(node, tree, source_code, visitor);
            metric.calculate_nom(node, tree, source_code, visitor);
            metric.nom -= 1; // Exclude the method itself
            metric.calculate_cc(node);

            self.add_metric(metric);
        }
    }
}

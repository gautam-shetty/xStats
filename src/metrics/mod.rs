use crate::ts::{Node, TSParsers, Tree};
use crate::utils::get_file_name;
use crate::visitor::TreeVisitor;

pub fn get_node_group(language: &str, group_name: &str) -> Vec<&'static str> {
    const JAVA_DECISION_POINTS: &[&str] = &[
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
    ];

    const JAVA_DECISION_POINTS_SKIP_NODES: &[&str] = &[
        "class_declaration",
        "method_declaration",
        "constructor_declaration",
    ];

    const PYTHON_DECISION_POINTS: &[&str] = &[
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
    ];

    const PYTHON_DECISION_POINTS_SKIP_NODES: &[&str] = &["class_definition", "function_definition"];

    match (language, group_name) {
        ("Java", "decision_point_nodes") => JAVA_DECISION_POINTS.to_vec(),
        ("Python", "decision_point_nodes") => PYTHON_DECISION_POINTS.to_vec(),
        ("Java", "decision_point_skip_nodes") => JAVA_DECISION_POINTS_SKIP_NODES.to_vec(),
        ("Python", "decision_point_skip_nodes") => PYTHON_DECISION_POINTS_SKIP_NODES.to_vec(),
        _ => {
            eprintln!(
                "Unsupported language or group name: {} - {}",
                language, group_name
            );
            vec![]
        }
    }
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

pub struct CodeMetricsMap {
    pub metrics: std::collections::HashMap<String, CodeMetrics>,
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

    /// Generate metrics for the node - start and end positions, aloc, and broken status
    pub fn generate_simple_node_metrics(&mut self, visitor: &TreeVisitor, node: &Node) {
        self.load_range_aloc(node);
        self.is_broken = visitor.check_if_broken(*node);
    }

    /// Load the range and aloc of the node
    fn load_range_aloc(&mut self, node: &Node) {
        let (start, end) = (node.start_position(), node.end_position());
        self.start_row = start.row as u32 + 1;
        self.start_col = start.column as u32 + 1;
        self.end_row = end.row as u32 + 1;
        self.end_col = end.column as u32 + 1;

        self.aloc = (end.row - start.row + 1) as u32;
    }

    /// Load the parameter count of the node
    pub fn load_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    /// Calculate the number of empty lines in the node
    pub fn calculate_eloc(&mut self, visitor: &TreeVisitor, node: &Node) {
        self.eloc = visitor.count_empty_lines(*node) as u32;
    }

    pub fn calculate_cloc_dcloc(&mut self, visitor: &TreeVisitor, comment_nodes: &Vec<Node>) {
        let (cloc, dcloc) = visitor.count_comments(comment_nodes);
        self.cloc = cloc as u32;
        self.dcloc = dcloc as u32;
    }

    /// Calculate the number of imports in the node
    pub fn calculate_noi(&mut self, import_nodes: &Vec<Node>) {
        self.noi = import_nodes.len() as u32;
    }

    /// Calculate the number of classes in the node
    pub fn calculate_noc(&mut self, class_nodes: &Vec<Node>) {
        self.noc = class_nodes.len() as u32;
    }

    /// Calculate the number of methods in the node
    pub fn calculate_nom(&mut self, method_nodes: &Vec<Node>) {
        self.nom = method_nodes.len() as u32;
    }

    fn count_decision_points(
        &self,
        node: Node,
        decision_points: &Vec<&str>,
        skip_nodes: &Vec<&str>,
    ) -> usize {
        let mut count = 0;

        // Check if the child node is a decision point
        if decision_points.contains(&node.kind()) {
            count += 1;
        }

        // Traverse child nodes to count decision points
        if !skip_nodes.contains(&node.kind()) {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    count += self.count_decision_points(child, decision_points, skip_nodes);
                }
            }
        }

        count
    }

    /// Calculate the cyclomatic complexity of the node
    pub fn calculate_cc(&mut self, node: &Node) {
        let decision_points = get_node_group(&self.language, "decision_point_nodes");
        let skip_nodes = get_node_group(&self.language, "decision_point_skip_nodes");

        self.cc = self.count_decision_points(*node, &decision_points, &skip_nodes) as u32 + 1;
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
        let visitor = TreeVisitor::new(parsers, &language, source_code);

        let root_node = tree.root_node();
        let root_type = root_node.kind();
        let mut metric = CodeMetric::new(
            language,
            &file_path,
            get_file_name(&file_path),
            root_type.to_string(),
        );
        metric.generate_simple_node_metrics(&visitor, &root_node);
        metric.calculate_eloc(&visitor, &root_node);
        let (comment_nodes, import_nodes, class_nodes, method_nodes) =
            visitor.perform_base_query(&root_node, tree);

        metric.calculate_cloc_dcloc(&visitor, &comment_nodes);
        metric.calculate_noi(&import_nodes);
        metric.calculate_cc(&root_node);

        // let class_nodes = visitor.get_class_nodes(&root_node, tree, source_code);
        // metric.noc = class_nodes.len() as u32;
        metric.calculate_noc(&class_nodes);

        // let method_nodes = visitor.get_method_nodes(&root_node, tree, source_code);
        // metric.nom = method_nodes.len() as u32;
        metric.calculate_nom(&method_nodes);

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
        class_nodes: &Vec<Node>,
        visitor: &TreeVisitor,
    ) {
        for node in class_nodes {
            let node_type = node.kind();

            let class_name = visitor.get_class_name(node);

            let (comment_nodes, import_nodes, class_nodes, method_nodes) =
                visitor.perform_base_query(&node, tree);

            let mut metric =
                CodeMetric::new(&language, &file_path, class_name, node_type.to_string());
            metric.generate_simple_node_metrics(&visitor, &node);
            metric.calculate_eloc(visitor, node);
            metric.calculate_cloc_dcloc(&visitor, &comment_nodes);
            metric.calculate_noi(&import_nodes);
            metric.calculate_noc(&class_nodes);
            metric.noc -= 1; // Exclude the class itself
            metric.calculate_nom(&method_nodes);
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
        method_nodes: &Vec<Node>,
        visitor: &TreeVisitor,
    ) {
        for node in method_nodes {
            let node_type = node.kind();

            let method_name = visitor.get_method_name(node);

            let (comment_nodes, import_nodes, class_nodes, method_nodes) =
                visitor.perform_base_query(&node, tree);

            let mut metric =
                CodeMetric::new(&language, &file_path, method_name, node_type.to_string());
            metric.generate_simple_node_metrics(&visitor, &node);

            metric.calculate_eloc(visitor, node);
            metric.calculate_cloc_dcloc(&visitor, &comment_nodes);
            metric.calculate_noi(&import_nodes);
            metric.calculate_noc(&class_nodes);
            metric.calculate_nom(&method_nodes);
            metric.nom -= 1; // Exclude the method itself
            metric.calculate_cc(node);

            let parameters_count = visitor.count_parameters(node);
            metric.load_pc(parameters_count as u32);

            self.add_metric(metric);
        }
    }
}

impl CodeMetricsMap {
    pub fn new() -> CodeMetricsMap {
        CodeMetricsMap {
            metrics: std::collections::HashMap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, CodeMetrics> {
        self.metrics.iter()
    }

    pub fn add_metrics(&mut self, commit_id: String, metrics: CodeMetrics) {
        self.metrics.insert(commit_id, metrics);
    }

    pub fn get_metrics(&self, commit_id: &String) -> Option<&CodeMetrics> {
        self.metrics.get(commit_id)
    }

    pub fn add_default_metrics(&mut self, metrics: CodeMetrics) {
        self.metrics.insert("default".to_string(), metrics);
    }

    pub fn get_default_metrics(&self) -> Option<&CodeMetrics> {
        self.metrics.values().next()
    }
}

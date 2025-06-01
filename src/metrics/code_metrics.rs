use crate::config::Language;
use crate::ts::{Node, TSParsers, Tree};
use crate::utils::get_file_name;
use crate::visitor::TreeVisitor;

pub struct CodeMetaData {
    /// The programming language of the source file.
    pub language: Language,
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
}

pub struct CodeMetric {
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

pub struct CodeMetricBlock {
    pub meta_data: CodeMetaData,
    pub metric: CodeMetric,
}

impl CodeMetricBlock {
    pub fn new(
        language: Language,
        file_path: &String,
        node_name: String,
        node_type: String,
    ) -> CodeMetricBlock {
        CodeMetricBlock {
            meta_data: CodeMetaData {
                language: language,
                file_path: file_path.to_string(),
                node_name,
                node_type,
                start_row: 0,
                start_col: 0,
                end_row: 0,
                end_col: 0,
            },
            metric: CodeMetric {
                is_broken: false,
                aloc: 0,
                eloc: 0,
                cloc: 0,
                dcloc: 0,
                noi: 0,
                noc: 0,
                nom: 0,
                cc: 0,
                pc: 0,
            },
        }
    }

    /// Generate metrics for the node - start and end positions, aloc, and broken status
    pub fn generate_simple_node_metrics(&mut self, visitor: &TreeVisitor, node: &Node) {
        self.load_range_aloc(node);
        self.metric.is_broken = visitor.check_if_broken(*node);
    }

    /// Load the range and aloc of the node
    fn load_range_aloc(&mut self, node: &Node) {
        let (start, end) = (node.start_position(), node.end_position());
        self.meta_data.start_row = start.row as u32 + 1;
        self.meta_data.start_col = start.column as u32 + 1;
        self.meta_data.end_row = end.row as u32 + 1;
        self.meta_data.end_col = end.column as u32 + 1;

        self.metric.aloc = (end.row - start.row + 1) as u32;
    }

    /// Load the parameter count of the node
    pub fn load_pc(&mut self, pc: u32) {
        self.metric.pc = pc;
    }

    /// Calculate the number of empty lines in the node
    pub fn calculate_eloc(&mut self, visitor: &TreeVisitor, node: &Node) {
        self.metric.eloc = visitor.count_empty_lines(*node) as u32;
    }

    pub fn calculate_cloc_dcloc(&mut self, visitor: &TreeVisitor, comment_nodes: &Vec<Node>) {
        let (cloc, dcloc) = visitor.count_comments(comment_nodes);
        self.metric.cloc = cloc as u32;
        self.metric.dcloc = dcloc as u32;
    }

    /// Calculate the number of imports in the node
    pub fn calculate_noi(&mut self, import_nodes: &Vec<Node>) {
        self.metric.noi = import_nodes.len() as u32;
    }

    /// Calculate the number of classes in the node
    pub fn calculate_noc(&mut self, class_nodes: &Vec<Node>) {
        self.metric.noc = class_nodes.len() as u32;
    }

    /// Calculate the number of methods in the node
    pub fn calculate_nom(&mut self, method_nodes: &Vec<Node>) {
        self.metric.nom = method_nodes.len() as u32;
    }

    fn count_decision_points(
        &self,
        node: Node,
        decision_points: &[String],
        skip_nodes: &[String],
    ) -> usize {
        let mut count = 0;

        let node_kind = node.kind().to_string();

        // // Check if the child node is a decision point
        // if decision_points.contains(&node_kind) {
        //     println!("Found decision point: {}", node_kind);
        //     count += 1;
        // }

        // // Traverse child nodes to count decision points
        // if !skip_nodes.contains(&node_kind) {
        //     for i in 0..node.child_count() {
        //         if let Some(child) = node.child(i) {
        //             count += self.count_decision_points(child, decision_points, skip_nodes);
        //         }
        //     }
        // } else {
        //     println!("Skipping node: {}", node_kind);
        // }

        if skip_nodes.contains(&node_kind) {
            // Don't count this node, but still traverse its children
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    count += self.count_decision_points(child, decision_points, skip_nodes);
                }
            }
            return count;
        }

        // Count this node if it's a decision point
        if decision_points.contains(&node_kind) {
            count += 1;
        }

        // Traverse children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                count += self.count_decision_points(child, decision_points, skip_nodes);
            }
        }

        count
    }

    /// Calculate the cyclomatic complexity of the node
    pub fn calculate_cc(&mut self, node: &Node) {
        let decision_points =
            get_node_group(self.meta_data.language.clone(), "decision_point_nodes");
        let skip_nodes =
            get_node_group(self.meta_data.language.clone(), "decision_point_skip_nodes");

        self.metric.cc =
            self.count_decision_points(*node, &decision_points, &skip_nodes) as u32 + 1;
    }
}

pub struct CodeMetrics {
    pub metric_blocks: Vec<CodeMetricBlock>,
}

impl CodeMetrics {
    pub fn new() -> CodeMetrics {
        CodeMetrics {
            metric_blocks: Vec::new(),
        }
    }

    fn add_metric_block(&mut self, code_metric_block: CodeMetricBlock) {
        self.metric_blocks.push(code_metric_block);
    }

    pub fn generate_root_metrics(
        &mut self,
        parsers: &TSParsers,
        source_code: &str,
        language: Language,
        file_path: &String,
        tree: &Tree,
    ) {
        let visitor = TreeVisitor::new(parsers, language, source_code);

        let root_node = tree.root_node();
        let root_type = root_node.kind();
        let mut metric_block = CodeMetricBlock::new(
            language,
            &file_path,
            get_file_name(&file_path),
            root_type.to_string(),
        );
        metric_block.generate_simple_node_metrics(&visitor, &root_node);
        metric_block.calculate_eloc(&visitor, &root_node);
        let (comment_nodes, import_nodes, class_nodes, method_nodes) =
            visitor.perform_base_query(&root_node, tree);

        metric_block.calculate_cloc_dcloc(&visitor, &comment_nodes);
        metric_block.calculate_noi(&import_nodes);
        metric_block.calculate_cc(&root_node);

        // let class_nodes = visitor.get_class_nodes(&root_node, tree, source_code);
        // metric.noc = class_nodes.len() as u32;
        metric_block.calculate_noc(&class_nodes);

        // let method_nodes = visitor.get_method_nodes(&root_node, tree, source_code);
        // metric.nom = method_nodes.len() as u32;
        metric_block.calculate_nom(&method_nodes);

        self.add_metric_block(metric_block);

        self.generate_class_metrics(
            &parsers,
            &source_code,
            language,
            file_path.to_string(),
            &tree,
            &class_nodes,
            &visitor,
        );
        self.generate_function_metrics(
            &parsers,
            &source_code,
            language,
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
        language: Language,
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

            let mut metric_block =
                CodeMetricBlock::new(language, &file_path, class_name, node_type.to_string());
            metric_block.generate_simple_node_metrics(&visitor, &node);
            metric_block.calculate_eloc(visitor, node);
            metric_block.calculate_cloc_dcloc(&visitor, &comment_nodes);
            metric_block.calculate_noi(&import_nodes);
            metric_block.calculate_noc(&class_nodes);
            metric_block.metric.noc -= 1; // Exclude the class itself
            metric_block.calculate_nom(&method_nodes);
            metric_block.calculate_cc(node);

            self.add_metric_block(metric_block);
        }
    }

    pub fn generate_function_metrics(
        &mut self,
        parsers: &TSParsers,
        source_code: &str,
        language: Language,
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

            let mut metric_block =
                CodeMetricBlock::new(language, &file_path, method_name, node_type.to_string());
            metric_block.generate_simple_node_metrics(&visitor, &node);

            metric_block.calculate_eloc(visitor, node);
            metric_block.calculate_cloc_dcloc(&visitor, &comment_nodes);
            metric_block.calculate_noi(&import_nodes);
            metric_block.calculate_noc(&class_nodes);
            metric_block.calculate_nom(&method_nodes);
            metric_block.metric.nom -= 1; // Exclude the method itself
            metric_block.calculate_cc(node);

            let parameters_count = visitor.count_parameters(node);
            metric_block.load_pc(parameters_count as u32);

            self.add_metric_block(metric_block);
        }
    }
}

pub fn get_node_group(language: Language, group_name: &str) -> Vec<String> {
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

    let vec = match (&language, group_name) {
        (Language::Java, "decision_point_nodes") => JAVA_DECISION_POINTS,
        (Language::Python, "decision_point_nodes") => PYTHON_DECISION_POINTS,
        (Language::Java, "decision_point_skip_nodes") => JAVA_DECISION_POINTS_SKIP_NODES,
        (Language::Python, "decision_point_skip_nodes") => PYTHON_DECISION_POINTS_SKIP_NODES,
        _ => {
            eprintln!(
                "Unsupported language or group name: {} - {}",
                language, group_name
            );
            &[]
        }
    };

    vec.iter().map(|s| s.to_string()).collect()
}

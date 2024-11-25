use designitex::metrics::CodeMetrics;
use designitex::parser::{TSParsers, Tree};
use std::path::Path;

fn main() {
    let target_files = vec![
        ("Java", "example/example.java"),
        ("Python", "example/example.py"),
    ];

    let mut parsers = TSParsers::new();

    for (language_name, file_path) in target_files {
        if Path::new(file_path).exists() {
            if let Some(tree) = parsers.generate_tree(language_name, file_path) {
                println!("{} tree generated successfully!", language_name);
                println!("Root node: {}", tree.root_node().to_sexp());

                let mut metrics = CodeMetrics::new();
                calculate_loc(&tree, &mut metrics);
                println!("{} LOC: {}", language_name, metrics.loc);
            } else {
                println!("Failed to generate {} tree.", language_name);
            }
        } else {
            println!("{} file not found: {}", language_name, file_path);
        }
    }
}

fn calculate_loc(tree: &Tree, metrics: &mut CodeMetrics) {
    let root_node = tree.root_node();
    let start_line = root_node.start_position().row;
    let end_line = root_node.end_position().row;
    metrics.loc = (end_line - start_line + 1) as u32;
}

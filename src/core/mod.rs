use crate::metrics::CodeMetrics;
use crate::parser::TSParsers;
use crate::utils::{save_to_csv, traverse_dir};

pub struct XStats {
    target_dir: String,
    output_dir: String,
    parsers: TSParsers,
    metrics: CodeMetrics,
}

impl XStats {
    pub fn new(target_dir: String, output_dir: String) -> Self {
        let parsers = TSParsers::new();
        Self {
            target_dir,
            output_dir,
            parsers,
            metrics: CodeMetrics::new(),
        }
    }

    pub fn run(&mut self) {
        let files = traverse_dir(&self.target_dir);
        for file in files {
            if let Some((language, tree, source_code)) = self.parsers.generate_tree(&file) {
                self.metrics
                    .generate_root_metrics(language.to_string(), file.clone(), &tree);
                self.metrics.generate_function_metrics(
                    &self.parsers,
                    &source_code,
                    language.to_string(),
                    file.clone(),
                    &tree,
                );
            }
        }
    }

    pub fn save_data(&self) {
        self.save_metrics();
    }

    pub fn save_metrics(&self) {
        let output_file = format!("{}/metrics.csv", self.output_dir);
        let mut data = Vec::new();
        data.push(vec![
            "language".to_string(),
            "file_path".to_string(),
            "node_name".to_string(),
            "node_type".to_string(),
            "start_row".to_string(),
            "start_col".to_string(),
            "end_row".to_string(),
            "end_col".to_string(),
            "aloc".to_string(),
            "cc".to_string(),
            "pc".to_string(),
        ]); // Add header row
        for metric in &self.metrics.metrics {
            data.push(vec![
                metric.language.clone(),
                metric.file_path.clone(),
                metric.node_name.clone(),
                metric.node_type.clone(),
                metric.start_row.to_string(),
                metric.start_col.to_string(),
                metric.end_row.to_string(),
                metric.end_col.to_string(),
                metric.aloc.to_string(),
                metric.cc.to_string(),
                metric.pc.to_string(),
            ]);
        }

        if save_to_csv(&output_file, data).is_ok() {
            println!("Code metrics saved at {}", output_file);
        } else {
            println!("Failed to save metrics to CSV");
        }
    }
}

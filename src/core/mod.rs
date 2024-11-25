use crate::metrics::CodeMetrics;
use crate::parser::TSParsers;
use crate::utils::{save_to_csv, traverse_dir};

pub struct DesigniteX {
    target_dir: String,
    output_dir: String,
    parsers: TSParsers,
    metrics: Vec<CodeMetrics>,
}

impl DesigniteX {
    pub fn new(target_dir: String, output_dir: String) -> Self {
        let parsers = TSParsers::new();
        Self {
            target_dir,
            output_dir,
            parsers,
            metrics: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let files = traverse_dir(&self.target_dir);
        for file in files {
            if let Some(tree) = self.parsers.generate_tree(&file) {
                let mut metrics = CodeMetrics::new(file);
                metrics.calculate_loc(&tree);
                self.add_metrics(metrics);
            }
        }
    }

    pub fn add_metrics(&mut self, metrics: CodeMetrics) {
        self.metrics.push(metrics);
    }

    pub fn save_data(&self) {
        self.save_metrics();
    }

    pub fn save_metrics(&self) {
        let output_file = format!("{}/metrics.csv", self.output_dir);
        let mut data = Vec::new();
        data.push(vec!["file_path".to_string(), "loc".to_string()]); // Add header row
        for metric in &self.metrics {
            data.push(vec![metric.file_path.clone(), metric.loc.to_string()]);
        }

        if save_to_csv(&output_file, data).is_ok() {
            println!("Code metrics saved at {}", output_file);
        } else {
            println!("Failed to save metrics to CSV");
        }
    }
}

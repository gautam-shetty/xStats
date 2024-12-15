mod code_metrics;
pub use code_metrics::CodeMetrics;

pub struct CodeMetricsMap {
    pub metrics: std::collections::HashMap<String, CodeMetrics>,
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

    pub fn get_table(&self, name: Option<&str>) -> Vec<Vec<String>> {
        let mut table = Vec::new();
        // Add header row
        table.push(vec![
            "language".to_string(),
            "file_path".to_string(),
            "start_row".to_string(),
            "start_col".to_string(),
            "end_row".to_string(),
            "end_col".to_string(),
            "node_name".to_string(),
            "node_type".to_string(),
            "is_broken".to_string(),
            "aloc".to_string(),
            "eloc".to_string(),
            "cloc".to_string(),
            "dcloc".to_string(),
            "noi".to_string(),
            "noc".to_string(),
            "nom".to_string(),
            "cc".to_string(),
            "pc".to_string(),
        ]);

        let metrics = if let Some(name) = name {
            self.get_metrics(&name.to_string())
        } else {
            self.get_default_metrics()
        };

        if let Some(metrics) = metrics {
            for block in &metrics.metric_blocks {
                table.push(vec![
                    block.meta_data.language.clone(),
                    block.meta_data.file_path.clone(),
                    block.meta_data.start_row.to_string(),
                    block.meta_data.start_col.to_string(),
                    block.meta_data.end_row.to_string(),
                    block.meta_data.end_col.to_string(),
                    block.meta_data.node_name.clone(),
                    block.meta_data.node_type.clone(),
                    block.metric.is_broken.to_string(),
                    block.metric.aloc.to_string(),
                    block.metric.eloc.to_string(),
                    block.metric.cloc.to_string(),
                    block.metric.dcloc.to_string(),
                    block.metric.noi.to_string(),
                    block.metric.noc.to_string(),
                    block.metric.nom.to_string(),
                    block.metric.cc.to_string(),
                    block.metric.pc.to_string(),
                ]);
            }
        }

        table
    }
}

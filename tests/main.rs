use xstats::core;
mod expected;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES_DIR: &str = "tests/samples";

    #[test]
    fn metric_test_example1() {
        let target_dir = format!("{}/example1", SAMPLES_DIR);
        let output_dir = SAMPLES_DIR.to_string();

        let mut xstats = core::XStats::new(target_dir, output_dir);
        xstats.run_default();
        let metrics: Vec<Vec<String>> = xstats.metrics_map.get_table(None);
        let metrics_ref: Vec<Vec<&str>> = metrics
            .iter()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .collect();
        let metrics_slice: Vec<&[&str]> = metrics_ref.iter().map(|v| v.as_slice()).collect();
        assert_eq!(
            metrics_slice.as_slice(),
            expected::EXPECTED_METRICS_EXAMPLE1
        );
    }
}

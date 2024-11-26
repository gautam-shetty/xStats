use designitex::core;

fn main() {
    // let target_dir = "tmp/example";
    let target_dir = "tmp/refactoring-toy-example";

    let output_dir = "tmp/op";

    let mut designitex = core::DesigniteX::new(target_dir.to_string(), output_dir.to_string());
    designitex.run();
    designitex.save_data();
}

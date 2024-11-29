use clap::Parser;
use std::time::Instant;
use xstats::core;

#[derive(Parser)]
#[clap(version = "1.0", author = "Gautam Shetty")]
struct Opts {
    #[clap(short = 't', long = "target")]
    target: String,

    #[clap(short = 'o', long = "output")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut designitex = core::XStats::new(opts.target, opts.output);
    let start = Instant::now();
    designitex.run();
    let duration = start.elapsed();

    println!("Time taken to execute run: {:?}", duration);
    designitex.save_data();
}

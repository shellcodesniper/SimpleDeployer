mod lib;
use lib::config;

fn main() {
    println!("Hello, world!");
    let _ = config::parser::ParsedConfig::new(Some("./bin/sampleConfig.cfg"));
}

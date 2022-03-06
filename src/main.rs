use std::env;
use std::fs;
use clap::Parser;

mod config;
mod drivers;

/// Environmental monitor and fan controler
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "/etc/envmon/envmon.conf")]
    config: String,
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.config.clone())
        .expect(&format!("config {} can't be opened", args.config));
    let config: config::Config = serde_yaml::from_str(&contents)
        .expect("config can't be parsed");
    drivers::load();
    println!("{:#?}",config);
}

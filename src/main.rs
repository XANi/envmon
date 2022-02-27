use std::env;
use std::fs;
use clap::Parser;

mod config;
/// Environmental monitor and fan controler
/// test
///
#[derive(Parser, Debug,Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value = "/etc/envmon/envmon.conf")]
    config: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    zake: u8,
}


fn main() {
    let s = "
---
pwm_drivers:
  - linux://it87.2608/hwmon/hwmon2
  - linux://it87.2608/hwmon/hwmon3
";
    let args = Args::parse();

    let contents = fs::read_to_string(args.config.clone()).expect(&format!("config {} can't be opened", args.config));
    let config: config::Config = serde_yaml::from_str(&s).unwrap();
    println!("{:#?}",config);
}

use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hash;
use clap::Parser;
use crate::drivers::PWM;
use crate::drivers::Temp;
use crate::drivers::Speed;

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
    let PWMDrivers: HashMap<String, Box<dyn PWM>> = HashMap::new();
    let TempDrivers: HashMap<String, Box<dyn Temp>> = HashMap::new();
    let SpeedDrivers: HashMap<String, Box<dyn Speed>> = HashMap::new();


    drivers::load().unwrap();
    println!("{:#?}",config);
}

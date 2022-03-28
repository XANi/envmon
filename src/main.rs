use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hash;
use clap::Parser;
use crate::drivers::PWM;
use crate::drivers::Temp;
use crate::drivers::Speed;
use std::{thread,error::Error, time::Duration};
mod config;
mod drivers;
use anyhow::*;
use log::*;
use signal_hook::*;
use signal_hook::iterator::Signals;
/// Environmental monitor and fan controler
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "/etc/envmon/envmon.conf")]
    config: String,
}


fn main() {
    let args = Args::parse();
    //env_logger::init();
    let contents = fs::read_to_string(args.config.clone())
        .expect(&format!("config {} can't be opened", args.config));
    let config: config::Config = serde_yaml::from_str(&contents)
        .expect("config can't be parsed");
    let mut pwm_drivers: HashMap<String, Box<dyn PWM>> = HashMap::new();
    let mut temp_drivers: HashMap<String, Box<dyn Temp>> = HashMap::new();
    let mut speed_drivers: HashMap<String, Box<dyn Speed>> = HashMap::new();
    for pwm in  config.pwm_drivers.as_ref()
        .expect("no pwm drivers in config")
        .iter() {
        let pwm_driver = drivers::pwm::load(pwm.1)
            .expect(&format!("could not load pwm driver from[{}]",pwm.1.clone()));
        pwm_drivers.insert(pwm.0.to_string(), pwm_driver);
    }
    for temp in  config.temperature_sensors.as_ref()
        .expect("no temp drivers in config")
        .iter() {
        let temp_driver = drivers::temp::load(temp.1)
            .expect(&format!("could not load temp driver from[{}]",temp.1.clone()));
        temp_drivers.insert(temp.0.to_string(), temp_driver);
    }
    for speed in  config.speed_sensors.as_ref()
        .expect("no speed drivers in config")
        .iter() {
        let speed_driver = drivers::speed::load(speed.1)
            .expect(&format!("could not load speed driver from[{}]",speed.1.clone()));
        speed_drivers.insert(speed.0.to_string(), speed_driver);
    }
    let mut signals = Signals::new(        signal_hook::consts::TERM_SIGNALS)
        .expect("couldn't get signals");

    // TODO that should probably have shared lock with the PWM modules and exit when they finish(or after deadman's timeout)
    // not just sleep for a second and hope for best..
    thread::spawn(move || {
        println!("in thread");
        for sig in signals.forever() {
            println!("Restoring old config");
            thread::sleep(Duration::from_millis(1000));
            std::process::exit(0);
        }
    });
    while 1 == 1 {
        for temp in temp_drivers.iter() {
            println!("{} : {}", temp.0, temp.1.read().expect("can't read sensor"))
        }
        for speed in speed_drivers.iter() {
            println!("{} : {}", speed.0, speed.1.read().expect("can't read sensor"))
        }
        thread::sleep(Duration::from_millis(4000));
    }
    println!("{:#?}",config);
}

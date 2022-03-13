use std::collections::HashMap;
use serde::Deserialize;


//#[derive(Debug)]
#[derive(Deserialize,Debug)]
pub struct Config {
     pwm_drivers: Option<HashMap<String,String>>,
     temperature_sensors: Option<HashMap<String,String>>,
}

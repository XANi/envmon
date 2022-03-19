use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize,Debug,Clone)]
pub struct SensorMap {
     pub pwm: String,
     pub min: f32,
     pub max: f32,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Config {
     pub pwm_drivers: Option<HashMap<String,String>>,
     pub temperature_sensors: Option<HashMap<String,String>>,
     pub speed_sensors: Option<HashMap<String,String>>,
     pub sensor_map: HashMap<String, SensorMap>,
}

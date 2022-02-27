use serde::Deserialize;


//#[derive(Debug)]
#[derive(Deserialize,Debug)]
pub struct Config {
     pwm_drivers: Option<Vec<String>>,
     temperature_sensors: Option<Vec<String>>,
}

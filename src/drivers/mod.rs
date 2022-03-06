use std::error::Error;
use std::error;
mod pwm;


pub trait PWM {
    fn read(&self) -> Result<u8, Box<dyn error::Error>>;
    fn write(&self,pwm: u8) -> Result<(),Box<dyn error::Error>>;
}

pub fn load() {
    let pwm1 = pwm::load("sysfs://it87/pwm3");

    
}


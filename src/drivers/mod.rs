use std::error::Error;
use std::error;
mod pwm;


pub trait PWM {
    fn read(&self) -> Result<u8, Box<dyn error::Error>>;
    fn write(&self,pwm: u8) -> Result<(),Box<dyn error::Error>>;
}

pub fn load() -> Result<(), Box<dyn Error>> {
    let pwm1 = pwm::load("sysfs://it87.2608/hwmon3/pwm3")?;
    let val = pwm1.read()?;
    print!("\npwm: {}\n",val);
    Ok(())



    
}


use std::error::Error;
use std::error;
mod pwm;
mod temp;


pub trait PWM {
    fn read(&self) -> Result<u8, Box<dyn error::Error>>;
    fn write(&self,pwm: u8) -> Result<(),Box<dyn error::Error>>;
}
pub trait Temp {
    fn read(&self) -> Result<u32, Box<dyn Error>>;
}
pub fn load() -> Result<(), Box<dyn Error>> {
    let pwm1 = pwm::load("sysfs://it87.2608/hwmon3/pwm3")?;
    let temp1 = temp::load("sysfs://nvme0n1/hwmon1/temp1")?;
    let val = pwm1.read()?;
    let val2 = temp1.read()?;
    print!("\npwm: {}\n",val);
    print!("\ntemp: {}\n",val2);
    Ok(())

}


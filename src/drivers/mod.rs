use std::error::Error;
use std::{error, fmt};
extern crate derive_more;
use derive_more::{Add, Display, From, Into,FromStr};
mod pwm;
mod temp;


// temp in thousands of kelvin

#[derive(PartialEq, From, FromStr, Add, Into)]
// temperature in milikelvins (so 10C is 273160)
pub struct TempMiliK(u32);

impl fmt::Display for TempMiliK {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:.2} C", ((self.0 as f32)/1000.0)-273.15)
        //write!(f, "{}", (self.0 as f32)/1000.0)
    }
}


pub trait PWM {
    fn read(&self) -> Result<u8, Box<dyn error::Error>>;
    fn write(&self,pwm: u8) -> Result<(),Box<dyn error::Error>>;
}
pub trait Temp {
    fn read(&self) -> Result<TempMiliK, Box<dyn Error>>;
}
pub trait Speed {
    fn read(&self) -> Result<u32, Box<dyn Error>>;
}
pub fn load() -> Result<(), Box<dyn Error>> {
    let pwm1 = pwm::load("sysfs://it87.2608/hwmon3/pwm3").unwrap();
    let temp1 = temp::load("sysfs://nvme0n1/hwmon1/temp1")?;
    let val = pwm1.read()?;
    let val2 = temp1.read()?;
    print!("\npwm: {}\n",val);
    print!("\ntemp: {}\n",val2);
    Ok(())

}


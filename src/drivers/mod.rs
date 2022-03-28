use std::error::Error;
use std::{error, fmt};
extern crate derive_more;
use derive_more::{Add, Display, From, Into,FromStr};
pub mod pwm;
pub mod temp;
pub mod speed;

use anyhow::{Context, Result};

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
impl From<TempMiliK> for f32 {
    fn from(temp: TempMiliK) -> Self {
        return (temp.0 as f32) / 1000.0
    }
}



pub trait PWM {
    fn read(&self) -> Result<u8>;
    fn write(&self,pwm: u8) -> Result<()>;
    fn cleanup(&self);
}
pub trait Temp {
    fn read(&self) -> Result<TempMiliK>;
}
pub trait Speed {
    fn read(&self) -> Result<u32>;
}

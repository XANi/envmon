use std::error::Error;
use std::str;
use std::fs;
use std::num::ParseIntError;
use url::Url;
use std::error;
use crate::drivers::PWM;

pub struct PwmSysfs {
    path: String,
}
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub fn init(url: Url) -> Result<impl PWM> {
    let pwm = PwmSysfs{ path: url.path().to_string() };
    print!("path: {:?}", url.path_segments());
    return Ok(pwm)

}

// impl PWM for PwmSysfs {
//     fn read(&self) ->Result<u8> {
//         let result = fs::read_to_string(self.path.clone())?.parse::<u8>()?;
//         return Ok(result);
//     }
//     fn write(&self, pwm: u8) -> Result<()> {
//         return Ok(())
//     }
// }
//

impl PWM for PwmSysfs {
    fn read(&self) -> std::result::Result<u8, Box<dyn Error>> {
        let result = fs::read_to_string(self.path.clone())?.parse::<u8>()?;
        return Ok(result);
    }

    fn write(&self, pwm: u8) -> std::result::Result<(), Box<dyn Error>> {
        return Ok(())
    }
}

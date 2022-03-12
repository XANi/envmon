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

pub fn init(url: Url) -> Result<impl PWM,Box<dyn Error>> {
    let path= url.path().to_string();
    let split_path= Vec::from_iter(path.split("/"));
    if split_path.len() < 3 {
         Err(format!("too short path(min 2 parts): {:?}", split_path))?
    }
    // /sys/devices/platform/it87.2608/hwmon/hwmon3/pwm3
    print!("{} -> {:?}\n",path,split_path);
    let host = url.host_str().ok_or("")?;
    let path = format!("/sys/devices/platform/{}/hwmon/{}/{}",host , split_path[1], split_path[2]);
    let pwm = PwmSysfs{ path: path };
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
    fn read(&self) -> Result<u8, Box<dyn Error>> {
        let result = fs::read_to_string(self.path.clone())
            .expect(&format!("file {} can't be opened",self.path.clone()))
            .trim().parse::<u8>()
            .expect(&format!("could not parse PWM at {}", self.path));
        return Ok(result);
    }

    fn write(&self, pwm: u8) -> std::result::Result<(), Box<dyn Error>> {
        return Ok(())
    }
}

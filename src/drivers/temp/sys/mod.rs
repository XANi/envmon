use std::error::Error;
use std::str;
use std::fs;
use std::num::ParseIntError;
use url::Url;
use std::error;

use crate::drivers::Temp;

pub struct TempSysfs {
    path: String,
}

pub fn init(url: Url) -> Result<impl Temp,Box<dyn Error>> {
    let path= url.path().to_string();
    let split_path= Vec::from_iter(path.split("/"));
    let host = url.host_str().ok_or("")?;
    let path: String;
    if host.starts_with("nvme") {
        // /sys/block/nvme0n1/device/hwmon1/temp1_input
        path = format!(
            "/sys/block/{}/device/{}/{}_input",
            host ,
            split_path[1],
            split_path[2]);
    } else {
        // /sys/devices/platform/it87.2608/hwmon/hwmon3/temp2_input
        path = format!(
            "/sys/devices/platform/{}/hwmon/{}/{}_input",
            host ,
            split_path[1],
            split_path[2]);
    }
    if split_path.len() < 3 {
         Err(format!("too short path(min 2 parts): {:?}", split_path))?
    }
    print!("{} -> {:?}\n",path,split_path);

    let pwm = TempSysfs{ path: path };
    return Ok(pwm)

}

// impl PWM for PwmSysfs {
//     fn read(&/sys/devices/platform/it87.2608/hwmon/hwmon3self) ->Result<u8> {
//         let result = fs::read_to_string(self.path.clone())?.parse::<u8>()?;
//         return Ok(result);
//     }
//     fn write(&self, pwm: u8) -> Result<()> {
//         return Ok(())
//     }
// }
//

impl Temp for TempSysfs {
    fn read(&self) -> Result<u32, Box<dyn Error>> {
        let result = fs::read_to_string(self.path.clone())
            .expect(&format!("file {} can't be opened",self.path.clone()))
            .trim().parse::<u32>()
            .expect(&format!("could not parse PWM at {}", self.path));
        return Ok(result);
    }

}

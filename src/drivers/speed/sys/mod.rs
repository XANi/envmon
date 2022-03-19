use std::error::Error;
use std::str;
use std::fs;
use std::num::ParseIntError;
use url::Url;
use std::error;

use crate::drivers::Speed;
use anyhow::*;
pub struct SpeedSysfs {
    path: String,
}

pub fn init(url: Url) -> Result<impl Speed> {
    let path= url.path().to_string();
    let split_path= Vec::from_iter(path.split("/"));
    if split_path.len() < 3 {
       bail!("too short path(min 2 parts): {:?}", split_path);
    }
    // /sys/devices/platform/it87.2608/hwmon/hwmon3/Speed3
    let host = url.host_str().context("no host part in url")?;
    // /sys/devices/platform/it87.2608/hwmon/hwmon3/fan1_input
    let path = format!("/sys/devices/platform/{}/hwmon/{}/{}",host , split_path[1], split_path[2]);
    let speed = SpeedSysfs{ path: path };
    return Ok(speed)

}

// impl Speed for SpeedSysfs {
//     fn read(&self) ->Result<u8> {
//         let result = fs::read_to_string(self.path.clone())?.parse::<u8>()?;
//         return Ok(result);
//     }
//     fn write(&self, Speed: u8) -> Result<()> {
//         return Ok(())
//     }
// }
//

impl Speed for SpeedSysfs {
    fn read(&self) -> Result<u32> {
        let result = fs::read_to_string(self.path.clone())
            .expect(&format!("file {} can't be opened",self.path.clone()))
            .trim().parse::<u32>()
            .expect(&format!("could not parse Speed at {}", self.path));
        return Ok(result);
    }
}

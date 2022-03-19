use std::error::Error;
use std::str;
use std::fs;
use std::num::ParseIntError;
use url::Url;
use std::error;
use std::path::Path;
use anyhow::*;
use crate::drivers::{Temp, TempMiliK};

pub struct TempSysfs {
    path: String,
}

pub fn init(url: Url) -> Result<impl Temp> {
    let path= url.path().to_string();
    let split_path= Vec::from_iter(path.split("/"));
    let host = url.host_str().context("missing host")?;
    let path: String;
    if split_path.len() < 3 {
         bail!("too short path(min 2 parts): {:?}", split_path);
    }
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
    if ! Path::new(&path).exists() {
        bail!("path {} does not exist", path)
    }
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
    fn read(&self) -> Result<TempMiliK> {
        let mut res32 = fs::read_to_string(self.path.clone())
            .expect(&format!("file {} can't be opened",self.path.clone()))
            .trim().parse::<i32>()
            .expect(&format!("could not parse temp at {}", self.path));
        let result = TempMiliK((u32::try_from(res32 + 273150))? as u32);
        return Ok(result);
    }

}

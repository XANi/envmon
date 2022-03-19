use std::str;
use std::fs;
use std::num::ParseIntError;
use url::Url;
use std::error;
use anyhow::{anyhow,bail, Context, Result};
use crate::drivers::PWM;

pub struct PwmSysfs {
    ctrl_path: String,
    ctrl_old: String,
    enable_path: String,
    enable_old: String,
}

pub fn init(url: Url) -> Result<impl PWM> {
    let path= url.path().to_string();
    let split_path= Vec::from_iter(path.split("/"));
    if split_path.len() < 3 {
        //Err("fuck");
         bail!("too short path(min 2 parts): {:?}", split_path);
    }
    // /sys/devices/platform/it87.2608/hwmon/hwmon3/pwm3
    let host = url.host_str().context("missing host")?;
    let ctrl_path = format!("/sys/devices/platform/{}/hwmon/{}/{}",host , split_path[1], split_path[2]);
    let enable_path = format!("/sys/devices/platform/{}/hwmon/{}/{}_enable",host , split_path[1], split_path[2]);
    let enable_old = fs::read_to_string(enable_path.clone())
        .expect(&format!("can't open {}", enable_path.clone()));
    // PWM value should not matter
    // it does at least for it87 ;  it defines max speed in automatic mode
    // even tho it is not writeable when in auto mode
    let ctrl_old = fs::read_to_string(ctrl_path.clone())
        .expect(&format!("can't open {}", ctrl_path.clone()));
    fs::write(enable_path.clone(),"1")
        .expect(&format!("can't write to {}", enable_path.clone()));
    fs::write(ctrl_path.clone(),"255")
        .expect(&format!("can't write to {}", ctrl_path.clone()));
    let pwm = PwmSysfs{
        ctrl_path: ctrl_path,
        ctrl_old: ctrl_old,
        enable_path: enable_path,
        enable_old: enable_old
    };
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
    fn read(&self) -> Result<u8> {
        let result = fs::read_to_string(self.ctrl_path.clone())
            .expect(&format!("file {} can't be opened",self.ctrl_path.clone()))
            .trim().parse::<u8>()
            .expect(&format!("could not parse PWM at {}", self.ctrl_path));
        return Ok(result);
    }

    fn write(&self, pwm: u8) -> Result<()> {
        return Ok(())
    }
}
// best effort to set PWM to its old value
impl Drop for PwmSysfs {
    fn drop(&mut self) {
        println!("restoring PWM {} to {}", self.ctrl_path.clone(),self.ctrl_old.clone());
        println!("restoring control mode {} to {}", self.enable_path.clone(),self.enable_old.clone());
        // just in case for whatever reason it was set on manual before start, we set fans to max speed
        let _ = fs::write(self.ctrl_path.clone(),self.ctrl_old.clone());
        let _ = fs::write(self.enable_path.clone(),self.enable_old.clone());

    }
}
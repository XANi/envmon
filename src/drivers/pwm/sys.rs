use std::str;
use std::fs;
use std::num::ParseIntError;
use url::Url;
use std::error;
use std::thread;
use anyhow::{anyhow,bail, Context, Result};
use signal_hook::*;
use signal_hook::iterator::Signals;
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
    let enable_old = fs::read_to_string(&enable_path)
        .expect(&format!("can't open {}", &enable_path));
    // PWM value should not matter
    // it does at least for it87 ;  it defines max speed in automatic mode
    // even tho it is not writeable when in auto mode
    let ctrl_old = fs::read_to_string(&ctrl_path)
        .expect(&format!("can't open {}", &ctrl_path));
    //
    fs::write(&enable_path,"1")
        .expect(&format!("can't write to {}", &enable_path));
    fs::write(&ctrl_path,"255")
        .expect(&format!("can't write to {}", &ctrl_path));
    let pwm = PwmSysfs{
        ctrl_path: ctrl_path.clone(),
        ctrl_old: ctrl_old.clone(),
        enable_path: enable_path.clone(),
        enable_old: enable_old.clone()
    };
    let mut signals = Signals::new(        signal_hook::consts::TERM_SIGNALS)
        .expect("couldn't get signals");
    let _ = thread::Builder::new().name(format!("envmon {} fence", split_path[2])).spawn(move || {
        println!("in thread");
        for sig in signals.forever() {
            println!("restoring PWM {} to {}", &ctrl_path,&ctrl_old);
            println!("restoring control mode {} to {}", &enable_path,&enable_old);
            // just in case for whatever reason it was set on manual before start, we set fans to max speed
            let _ = fs::write(&ctrl_path,&ctrl_old);
            let _ = fs::write(&enable_path,&enable_old);
            return;
        }
    });


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
        let result = fs::read_to_string(&self.ctrl_path)
            .expect(&format!("file {} can't be opened",&self.ctrl_path))
            .trim().parse::<u8>()
            .expect(&format!("could not parse PWM at {}", &self.ctrl_path));
        return Ok(result);
    }

    fn write(&self, pwm: u8) -> Result<()> {
        return Ok(())
    }
    fn cleanup(&self) {
        println!("restoring PWM {} to {}", &self.ctrl_path,self.ctrl_old);
        println!("restoring control mode {} to {}", &self.enable_path,&self.enable_old);
        // just in case for whatever reason it was set on manual before start, we set fans to max speed
        let _ = fs::write(&self.ctrl_path,&self.ctrl_old);
        let _ = fs::write(&self.enable_path,&self.enable_old);
    }
}
// best effort to set PWM to its old value
impl Drop for PwmSysfs {
    fn drop(&mut self) {
       self.cleanup();
    }
}
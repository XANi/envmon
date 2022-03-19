use std::error;
use url::Url;
use crate::drivers::PWM;
use anyhow::{bail, Context, Result};
pub mod sys;
pub fn load(url: &str)  -> Result<Box<dyn PWM>>{
    let url = Url::parse(url)?;
    match url.scheme() {
        "sysfs" => {
            return  Ok(Box::new(sys::init(url)?));
        }
        _ => {
            bail!("plugin [{}] not supported", url.scheme());
        }

    }
}
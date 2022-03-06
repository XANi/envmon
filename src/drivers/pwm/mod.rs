use std::error;
use url::Url;
use crate::drivers::PWM;

pub mod sys;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub fn load(url: &str)  -> Result<Box<dyn PWM>>{
    let url = Url::parse(url)?;
    match url.scheme() {
        "sysfs" => {
            return  Ok(Box::new(sys::init(url)?));
        }
        _ => {
            return Err(format!("plugin [{}] not supported", url.scheme()).into())
        }

    }

}
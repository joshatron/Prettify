pub mod base64_converter;
pub mod json_converter;

use crate::error::Error;
use crate::options::Options;

pub trait Converter {
    fn name(&self) -> &str;
    fn options(&self) -> Vec<&str>;
    fn prettify(&self, input: &str, options: &Options) -> Result<String, Error>;
}

pub fn get_converters() -> Vec<Box<dyn Converter>> {
    vec![
        Box::new(json_converter::JsonConverter::new()),
        Box::new(base64_converter::Base64Converter::new()),
    ]
}

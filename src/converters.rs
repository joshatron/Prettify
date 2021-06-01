pub mod json_converter;

use crate::error::Error;
use crate::options::Options;

pub trait Converter {
    fn name(&self) -> &str;
    fn convert(&self, input: &String, options: &Options) -> Result<String, Error>;
}

pub fn get_converters() -> Vec<Box<dyn Converter>> {
    vec![Box::new(json_converter::JsonConverter::new())]
}

pub mod json_converter;

use crate::error::Error;
use crate::options::Options;

pub trait Converter {
    fn name(&self) -> &str;
    fn prettify(&self, input: &String, options: &Options) -> Result<String, Error>;
}

//TODO: Create XML converter
//TODO: Create base64 converter
pub fn get_converters() -> Vec<Box<dyn Converter>> {
    vec![Box::new(json_converter::JsonConverter::new())]
}

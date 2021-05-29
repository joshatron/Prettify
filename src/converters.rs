pub mod json;

pub trait Converter {
    fn name(&self) -> &str;
    fn can_convert(&self, input: &String) -> bool;
    fn convert(&self, input: &String) -> String;
}

pub fn get_converters() -> Vec<Box<dyn Converter>> {
    vec![Box::new(json::Json::new())]
}

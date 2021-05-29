use crate::converters::Converter;

pub struct Json {}

impl Json {
    pub fn new() -> Json {
        Json {}
    }
}

impl Converter for Json {
    fn name(&self) -> &str {
        "JSON"
    }

    fn can_convert(&self, input: &String) -> bool {
        false
    }

    fn convert(&self, input: &String) -> String {
        String::new()
    }
}

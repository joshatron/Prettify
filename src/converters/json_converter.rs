use crate::converters::Converter;
use crate::error::Error;
use crate::options::Options;
use json;

pub struct JsonConverter {}

impl JsonConverter {
    pub fn new() -> JsonConverter {
        JsonConverter {}
    }
}

impl Converter for JsonConverter {
    fn name(&self) -> &str {
        "JSON"
    }

    fn convert(&self, input: &String, options: &Options) -> Result<String, Error> {
        match json::parse(input) {
            Ok(parsed) => Ok(String::new()),
            Err(_) => Err(Error::Cannot_Convert),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_invalid() {
        let converter = JsonConverter::new();
        let options = Options::default();

        assert_eq!(
            converter.convert(&String::from("not json"), &options),
            Err(Error::Cannot_Convert)
        );
        assert_eq!(
            converter.convert(&String::from("{not quite json}"), &options),
            Err(Error::Cannot_Convert)
        );
    }

    #[test]
    fn test_convert_default() {
        let converter = JsonConverter::new();
        let options = Options::default();

        converter.convert(&String::from("{\"json\": true}"), &options);
        converter.convert(
            &String::from(
                "{\"json\":true,\"complex\":[\"field1\",\"field2\"],\"object\":{\"field3\":4}}",
            ),
            &options,
        );
    }
}

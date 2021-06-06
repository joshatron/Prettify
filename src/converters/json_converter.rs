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

    fn prettify(&self, input: &String, options: &Options) -> Result<String, Error> {
        match json::parse(input) {
            Ok(parsed) => Ok(json::stringify_pretty(parsed, options.indent_size.into())),
            Err(_) => Err(Error::CannotConvert),
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
            converter.prettify(&String::from("not json"), &options),
            Err(Error::CannotConvert)
        );
        assert_eq!(
            converter.prettify(&String::from("{not quite json}"), &options),
            Err(Error::CannotConvert)
        );
    }

    #[test]
    fn test_convert_default() {
        let converter = JsonConverter::new();
        let options = Options::default();

        assert_eq!(
            converter.prettify(&String::from(r#"{"json": true}"#), &options),
            Ok(String::from(
                r#"{
    "json": true
}"#
            ))
        );
        assert_eq!(
            converter.prettify(
                &String::from(
                    r#"{
  "json": false
}"#
                ),
                &options
            ),
            Ok(String::from(
                r#"{
    "json": false
}"#
            ))
        );
        assert_eq!(
            converter.prettify(
                &String::from(
                    r#"{"json":true,"complex":["field1","field2"],"object":{"field3":4}}"#,
                ),
                &options,
            ),
            Ok(String::from(
                r#"{
    "json": true,
    "complex": [
        "field1",
        "field2"
    ],
    "object": {
        "field3": 4
    }
}"#
            ))
        );
    }

    #[test]
    fn test_convert_different_indent_size() {
        let converter = JsonConverter::new();
        let mut options = Options::default();
        options.indent_size = 2;

        assert_eq!(
            converter.prettify(&String::from(r#"{"json":"string"}"#), &options),
            Ok(String::from(
                r#"{
  "json": "string"
}"#
            ))
        );
    }
}

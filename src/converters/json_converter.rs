use crate::converter_utils;
use crate::converters::Converter;
use crate::error::Error;
use crate::options::Options;
use json;
use std::collections::HashMap;

const INDENT_OPTION_NAME: &str = "indent";

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

    fn options(&self) -> Vec<&str> {
        vec![INDENT_OPTION_NAME]
    }

    fn prettify(&self, input: &str, options: &Options) -> Result<String, Error> {
        let json_options = JsonConverterOptions::from_options(&options.converter_specific);
        match json::parse(input) {
            Ok(parsed) => {
                if options.reverse {
                    Ok(json::stringify(parsed))
                } else {
                    Ok(json::stringify_pretty(parsed, json_options.indent))
                }
            }
            Err(_) => Err(Error::CannotConvert),
        }
    }
}

struct JsonConverterOptions {
    indent: u16,
}

impl JsonConverterOptions {
    pub fn default() -> JsonConverterOptions {
        JsonConverterOptions { indent: 4 }
    }

    pub fn from_options(converter_options: &HashMap<String, String>) -> JsonConverterOptions {
        JsonConverterOptions {
            indent: converter_utils::extract_u16(
                INDENT_OPTION_NAME,
                converter_options,
                JsonConverterOptions::default().indent,
            ),
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
        options
            .converter_specific
            .insert("indent".to_string(), "2".to_string());

        assert_eq!(
            converter.prettify(&String::from(r#"{"json":"string"}"#), &options),
            Ok(String::from(
                r#"{
  "json": "string"
}"#
            ))
        );
    }

    #[test]
    fn test_convert_reverse() {
        let converter = JsonConverter::new();
        let mut options = Options::default();
        options.reverse = true;

        assert_eq!(
            converter.prettify(
                &String::from(
                    r#"{
    "json":"string"
}"#
                ),
                &options
            ),
            Ok(String::from(r#"{"json":"string"}"#))
        );
    }
}

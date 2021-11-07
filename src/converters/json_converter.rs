use crate::converter_utils;
use crate::converters::Converter;
use crate::error::Error;
use crate::options::Options;
use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter, Styler};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::iter;

const INDENT_OPTION_NAME: &str = "indent";
const COLOR_OPTION_NAME: &str = "color";

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
        vec![INDENT_OPTION_NAME, COLOR_OPTION_NAME]
    }

    fn prettify(&self, input: &str, options: &Options) -> Result<String, Error> {
        let json_options = JsonConverterOptions::from_options(&options.converter_specific);

        match serde_json::from_str(input) {
            Ok(parsed) => format(&parsed, options, &json_options),
            Err(_) => Err(Error::CannotConvert),
        }
    }
}

fn format(
    parsed: &Value,
    options: &Options,
    json_options: &JsonConverterOptions,
) -> Result<String, Error> {
    if json_options.color {
        if options.reverse {
            let formatter = ColoredFormatter::with_styler(CompactFormatter {}, Styler::default());
            match formatter.to_colored_json_auto(parsed) {
                Ok(formatted) => Ok(formatted),
                Err(_) => Err(Error::CannotConvert),
            }
        } else {
            let indent = get_indent_str(json_options.indent);
            let formatter = ColoredFormatter::with_styler(
                PrettyFormatter::with_indent(indent.as_bytes()),
                Styler::default(),
            );
            match formatter.to_colored_json_auto(parsed) {
                Ok(formatted) => Ok(formatted),
                Err(_) => Err(Error::CannotConvert),
            }
        }
    } else {
        if options.reverse {
            Ok(parsed.to_string())
        } else {
            let indent = get_indent_str(json_options.indent);
            let buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
            let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
            parsed.serialize(&mut ser).unwrap();
            match String::from_utf8(ser.into_inner()) {
                Ok(formatted) => Ok(formatted),
                Err(_) => Err(Error::CannotConvert),
            }
        }
    }
}

fn get_indent_str(indent: u8) -> String {
    iter::repeat(" ").take(indent.into()).collect()
}

struct JsonConverterOptions {
    indent: u8,
    color: bool,
}

impl JsonConverterOptions {
    pub fn default() -> JsonConverterOptions {
        JsonConverterOptions {
            indent: 4,
            color: true,
        }
    }

    pub fn from_options(converter_options: &HashMap<String, String>) -> JsonConverterOptions {
        JsonConverterOptions {
            indent: converter_utils::extract_u8(
                INDENT_OPTION_NAME,
                converter_options,
                JsonConverterOptions::default().indent,
            ),
            color: converter_utils::extract_bool(
                COLOR_OPTION_NAME,
                converter_options,
                JsonConverterOptions::default().color,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_invalid_no_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("false"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("4"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: json_options,
        };

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
    fn test_convert_invalid_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("true"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("4"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: json_options,
        };

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
    fn test_convert_no_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("false"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("4"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: json_options,
        };

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
                    r#"{"hello":[{"json":true},{"complex":["field1","field2"]},{"object":{"field3":4}}]}"#,
                ),
                &options,
            ),
            Ok(String::from(
                r#"{
    "hello": [
        {
            "json": true
        },
        {
            "complex": [
                "field1",
                "field2"
            ]
        },
        {
            "object": {
                "field3": 4
            }
        }
    ]
}"#
            ))
        );
    }

    #[test]
    fn test_convert_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("true"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("4"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: json_options,
        };

        assert_eq!(
            converter.prettify(&String::from(r#"{"json": true}"#), &options),
            Ok(String::from("\u{1b}[1m{\u{1b}[0m\n    \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mjson\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: true\u{1b}[1m\n}\u{1b}[0m"))
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
            Ok(String::from("\u{1b}[1m{\u{1b}[0m\n    \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mjson\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: false\u{1b}[1m\n}\u{1b}[0m"))
        );
        assert_eq!(
            converter.prettify(
                &String::from(
                    r#"{"hello":[{"json":true},{"complex":["field1","field2"]},{"object":{"field3":4}}]}"#,
                ),
                &options,
            ),
            Ok(String::from("\u{1b}[1m{\u{1b}[0m\n    \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mhello\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: \u{1b}[1m[\u{1b}[0m\n        \u{1b}[1m{\u{1b}[0m\n            \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mjson\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: true\u{1b}[1m\n        }\u{1b}[0m,\n        \u{1b}[1m{\u{1b}[0m\n            \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mcomplex\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: \u{1b}[1m[\u{1b}[0m\n                \u{1b}[32m\"\u{1b}[0m\u{1b}[32mfield1\u{1b}[0m\u{1b}[32m\"\u{1b}[0m,\n                \u{1b}[32m\"\u{1b}[0m\u{1b}[32mfield2\u{1b}[0m\u{1b}[32m\"\u{1b}[0m\u{1b}[1m\n            ]\u{1b}[0m\u{1b}[1m\n        }\u{1b}[0m,\n        \u{1b}[1m{\u{1b}[0m\n            \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mobject\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: \u{1b}[1m{\u{1b}[0m\n                \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mfield3\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: 4\u{1b}[1m\n            }\u{1b}[0m\u{1b}[1m\n        }\u{1b}[0m\u{1b}[1m\n    ]\u{1b}[0m\u{1b}[1m\n}\u{1b}[0m"))
        );
    }

    #[test]
    fn test_convert_different_indent_size_no_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("false"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("2"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: json_options,
        };

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
    fn test_convert_different_indent_size_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("true"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("2"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: json_options,
        };

        assert_eq!(
            converter.prettify(&String::from(r#"{"json":"string"}"#), &options),
            Ok(String::from("\u{1b}[1m{\u{1b}[0m\n  \u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mjson\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m: \u{1b}[32m\"\u{1b}[0m\u{1b}[32mstring\u{1b}[0m\u{1b}[32m\"\u{1b}[0m\u{1b}[1m\n}\u{1b}[0m"))
        );
    }

    #[test]
    fn test_convert_reverse_no_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("false"));
        json_options.insert(String::from(INDENT_OPTION_NAME), String::from("4"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: true,
            converter_specific: json_options,
        };

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

    #[test]
    fn test_convert_reverse_color() {
        let converter = JsonConverter::new();
        let mut json_options = HashMap::new();
        json_options.insert(String::from(COLOR_OPTION_NAME), String::from("true"));
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: true,
            converter_specific: json_options,
        };

        assert_eq!(
            converter.prettify(
                &String::from(
                    r#"{
    "json":"string"
}"#
                ),
                &options
            ),
            Ok(String::from(
                "\u{1b}[1m{\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m\u{1b}[1;34mjson\u{1b}[0m\u{1b}[1;34m\"\u{1b}[0m:\u{1b}[32m\"\u{1b}[0m\u{1b}[32mstring\u{1b}[0m\u{1b}[32m\"\u{1b}[0m\u{1b}[1m}\u{1b}[0m"
            ))
        );
    }
}

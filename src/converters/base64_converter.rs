use crate::converters::Converter;
use crate::error::Error;
use crate::options::Options;
use base64::{decode, encode};
use std::str;

pub struct Base64Converter {}

impl Base64Converter {
    pub fn new() -> Base64Converter {
        Base64Converter {}
    }
}

impl Converter for Base64Converter {
    fn name(&self) -> &str {
        "Base64"
    }

    fn options(&self) -> Vec<&str> {
        vec![]
    }

    fn prettify(&self, input: &str, options: &Options) -> Result<String, Error> {
        if options.reverse {
            if options.input_type.eq_ignore_ascii_case(self.name()) {
                Ok(encode(input))
            } else {
                if options.verbose {
                    println!("You can only encode to base64 if you specify it as the converter directly.");
                }
                Err(Error::CannotConvert)
            }
        } else {
            match decode(input) {
                Ok(decoded) => u8s_to_string(decoded),
                Err(_) => Err(Error::CannotConvert),
            }
        }
    }
}

fn u8s_to_string(input: Vec<u8>) -> Result<String, Error> {
    match String::from_utf8(input) {
        Ok(string) => Ok(string),
        Err(_) => Err(Error::CannotConvert),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_decode_valid() {
        let converter = Base64Converter::new();
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: HashMap::new(),
        };

        assert_eq!(
            converter.prettify(&String::from("aGVsbG8gd29ybGQh"), &options),
            Ok(String::from("hello world!"))
        );
        assert_eq!(
            converter.prettify(&String::from("SGkkQA=="), &options),
            Ok(String::from("Hi$@"))
        );
    }

    #[test]
    fn test_decode_invalid() {
        let converter = Base64Converter::new();
        let options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: HashMap::new(),
        };

        assert_eq!(
            converter.prettify(&String::from("aGVsbG8gd29ybGQht"), &options),
            Err(Error::CannotConvert)
        );
        assert_eq!(
            converter.prettify(&String::from("aGVsbG8gd29ybGQh======="), &options),
            Err(Error::CannotConvert)
        );
        assert_eq!(
            converter.prettify(&String::from("aGVsbG8gd29ybG?h"), &options),
            Err(Error::CannotConvert)
        );
    }

    #[test]
    fn test_encode_valid_input_type() {
        let converter = Base64Converter::new();
        let mut options = Options {
            input_type: String::from("base64"),
            verbose: false,
            reverse: true,
            converter_specific: HashMap::new(),
        };

        assert_eq!(
            converter.prettify(&String::from("hello world!"), &options),
            Ok(String::from("aGVsbG8gd29ybGQh"))
        );
        assert_eq!(
            converter.prettify(&String::from(""), &options),
            Ok(String::from(""))
        );
        options.input_type = String::from("BASE64");
        assert_eq!(
            converter.prettify(&String::from("Hi$@"), &options),
            Ok(String::from("SGkkQA=="))
        );
    }

    #[test]
    fn test_encode_invalid_input_type() {
        let converter = Base64Converter::new();
        let mut options = Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: true,
            converter_specific: HashMap::new(),
        };

        assert_eq!(
            converter.prettify(&String::from("hello world!"), &options),
            Err(Error::CannotConvert)
        );
        options.input_type = String::from("JSON");
        assert_eq!(
            converter.prettify(&String::from(""), &options),
            Err(Error::CannotConvert)
        );
    }
}

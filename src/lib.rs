pub mod converters;
pub mod error;
pub mod options;

use self::options::Options;

pub fn prettify_default(input: &String) -> String {
    prettify(input, &Options::default())
}

pub fn prettify(input: &String, options: &Options) -> String {
    String::from(input)
}

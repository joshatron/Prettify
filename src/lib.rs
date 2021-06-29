pub mod converters;
pub mod error;
pub mod options;

use self::options::Options;

pub fn prettify_default(input: &String) -> Result<String, String> {
    prettify(input, &Options::default())
}

pub fn prettify(input: &str, options: &Options) -> Result<String, String> {
    for converter in converters::get_converters() {
        if should_try_converter(&options.input_type, converter.name()) {
            if options.verbose {
                println!("Trying to convert as {}", converter.name())
            };
            match converter.prettify(input, options) {
                Ok(output) => {
                    if options.verbose {
                        println!("Conversion successful!");
                    }
                    return Ok(output);
                }
                Err(_) => {
                    if options.verbose {
                        println!("Conversion failed.");
                    }
                }
            }
        }
    }

    Err(String::from(input))
}

fn should_try_converter(input_type: &str, converter_name: &str) -> bool {
    input_type.eq_ignore_ascii_case("all") || input_type.eq_ignore_ascii_case(converter_name)
}

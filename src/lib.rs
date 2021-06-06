pub mod converters;
pub mod error;
pub mod options;

use self::options::Options;

pub fn prettify_default(input: &String) -> String {
    prettify(input, &Options::default())
}

pub fn prettify(input: &String, options: &Options) -> String {
    for converter in converters::get_converters() {
        if options.verbose {
            println!("Trying to convert as {}", converter.name())
        };
        match converter.prettify(input, options) {
            Ok(output) => return output,
            Err(_) => {
                if options.verbose {
                    println!("Conversion failed.");
                }
            }
        }
    }

    String::from("")
}

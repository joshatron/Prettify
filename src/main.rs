use prettify;
extern crate clap;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("indent")
                .short("i")
                .long("indent")
                .value_name("INDENT")
                .help("Number of spaces to indent the output.")
                .takes_value(true)
                .default_value("4"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Increases logging level to show more detailed output."),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The minified string to parse.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let indent_str = matches.value_of("indent").unwrap();
    let indent = match indent_str.parse::<u8>() {
        Ok(i) => i,
        Err(_) => {
            println!(
                "Indent value '{}' is not valid. Defaulting to 4.",
                indent_str
            );
            4
        }
    };

    let options = prettify::options::Options {
        indent_size: indent,
        verbose: matches.occurrences_of("verbose") > 0,
    };

    let input = String::from(matches.value_of("INPUT").unwrap());

    let result = prettify::prettify(&input, &options);

    if result.is_empty() {
        println!("Input was not in any recognized format.")
    } else {
        println!("{}", result);
    }
}

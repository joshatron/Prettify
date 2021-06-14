use prettify;
use prettify::options::Options;
extern crate clap;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

fn main() {
    let matches = get_matches();

    let input = build_input(&matches);
    let options = build_options(&matches);

    let result = prettify::prettify(&input, &options);

    if result.is_empty() {
        println!("Input was not in any recognized format.");
        println!("{}", input);
    } else {
        println!("{}", result);
    }
}

//TODO: Add ability to get input from file
//TODO: Add ability to get input from stdin
fn build_input(matches: &ArgMatches) -> String {
    String::from(matches.value_of("INPUT").unwrap())
}

fn build_options(matches: &ArgMatches) -> Options {
    Options {
        indent_size: get_indent(matches),
        verbose: matches.occurrences_of("verbose") > 0,
    }
}

fn get_indent(matches: &ArgMatches) -> u8 {
    let indent_str = matches.value_of("indent").unwrap();
    match indent_str.parse::<u8>() {
        Ok(i) => i,
        Err(_) => {
            println!(
                "Indent value '{}' is not valid. Defaulting to 4.",
                indent_str
            );
            4
        }
    }
}

fn get_matches() -> ArgMatches<'static> {
    App::new(crate_name!())
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
        .get_matches()
}

use prettify;
use prettify::options::Options;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::process;
extern crate clap;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

fn main() {
    let matches = get_matches();

    let input = build_input(&matches);
    if input.is_empty() {
        eprintln!("No input given.");
        eprintln!("Please provide either a file name, the string in an arguement, or stdin.");
        process::exit(1);
    }

    let options = build_options(&matches);

    let mut inputs: Vec<&str> = Vec::new();
    if should_split_lines(&matches) {
        inputs = input.lines().collect();
    } else {
        inputs.push(&input);
    }

    let mut any_errors = false;
    for i in inputs {
        let result = prettify::prettify(i, &options);
        print_results(&result);
        if result.is_err() {
            any_errors = true;
        }
    }

    if any_errors {
        process::exit(1);
    }
}

fn should_split_lines(matches: &ArgMatches) -> bool {
    matches.occurrences_of("lines") > 0
}

fn print_results(result: &Result<String, String>) {
    match result {
        Ok(s) => println!("{}", s),
        Err(s) => {
            eprintln!("Input was not in any recognized format.");
            eprintln!("{}", s);
        }
    }
}

fn build_input(matches: &ArgMatches) -> String {
    match get_input_from_file(matches.value_of("file")) {
        Ok(s) => return s,
        Err(_) => (),
    }

    match get_input_from_arg(matches.value_of("INPUT")) {
        Ok(s) => return s,
        Err(_) => (),
    }

    match get_input_from_stdin() {
        Ok(s) => return s,
        Err(_) => (),
    }

    String::new()
}

fn get_input_from_file(file: Option<&str>) -> Result<String, String> {
    match file {
        Some(f) => {
            let contents = fs::read_to_string(f);
            match contents {
                Ok(s) => Ok(s),
                Err(_) => Err(String::new()),
            }
        }
        None => Err(String::new()),
    }
}

fn get_input_from_arg(arg: Option<&str>) -> Result<String, String> {
    match arg {
        Some(s) => Ok(String::from(s)),
        None => Err(String::new()),
    }
}

fn get_input_from_stdin() -> Result<String, String> {
    let mut input = String::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        input.push_str(&line);
        input.push('\n');
    }

    if !input.is_empty() {
        Ok(input)
    } else {
        Err(String::new())
    }
}

fn build_options(matches: &ArgMatches) -> Options {
    Options {
        input_type: String::from(matches.value_of("type").unwrap()),
        verbose: matches.occurrences_of("verbose") > 0,
        reverse: matches.occurrences_of("reverse") > 0,
        converter_specific: get_converter_specific(matches),
    }
}

fn get_converter_specific(matches: &ArgMatches) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("indent".to_string(), get_indent(matches).to_string());
    map.insert("color".to_string(), get_color(matches).to_string());

    map
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

fn get_color(matches: &ArgMatches) -> bool {
    matches.occurrences_of("no color") == 0
}

fn get_matches() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Type of data in the input. The options are JSON and all (default).")
                .takes_value(true)
                .default_value("all"),
        )
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
            Arg::with_name("no color")
                .short("c")
                .long("no-color")
                .help("Disables colored output."),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Increases logging level to show more detailed output."),
        )
        .arg(
            Arg::with_name("reverse")
                .short("r")
                .long("reverse")
                .help("Instead of prettifying, minimizes the input."),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Treat each line as a seperate input."),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File contents to prettify.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The minified string to parse.")
                .index(1),
        )
        .get_matches()
}

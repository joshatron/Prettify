//TODO: Add option for colorized output
//TODO: Add option to convert using specific converter
pub struct Options {
    pub input_type: String,
    pub indent_size: u8,
    pub verbose: bool,
    pub reverse: bool,
}

impl Options {
    pub fn default() -> Options {
        Options {
            input_type: String::from("all"),
            indent_size: 4,
            verbose: false,
            reverse: false,
        }
    }
}

use std::collections::HashMap;

//TODO: Add option for colorized output
pub struct Options {
    pub input_type: String,
    pub verbose: bool,
    pub reverse: bool,
    pub converter_specific: HashMap<String, String>,
}

impl Options {
    pub fn default() -> Options {
        Options {
            input_type: String::from("all"),
            verbose: false,
            reverse: false,
            converter_specific: HashMap::new(),
        }
    }
}

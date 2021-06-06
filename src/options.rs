pub struct Options {
    pub indent_size: u8,
    pub verbose: bool,
}

impl Options {
    pub fn default() -> Options {
        Options {
            indent_size: 4,
            verbose: false,
        }
    }
}

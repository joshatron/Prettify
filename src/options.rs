pub struct Options {
    pub indent_size: u8,
}

impl Options {
    pub fn default() -> Options {
        Options { indent_size: 4 }
    }
}

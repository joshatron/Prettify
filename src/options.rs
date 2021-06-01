pub struct Options {
    indent_size: usize,
}

impl Options {
    pub fn default() -> Options {
        Options { indent_size: 4 }
    }
}

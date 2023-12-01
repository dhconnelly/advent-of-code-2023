pub trait IndexAscii {
    fn ascii_at(&self, i: usize) -> char;
}

impl IndexAscii for &str {
    fn ascii_at(&self, i: usize) -> char {
        self.as_bytes()[i] as char
    }
}

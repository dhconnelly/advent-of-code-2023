use core::str::Lines;

pub type LineWindow<'a> = (Option<&'a str>, &'a str, Option<&'a str>);

pub struct Windows<'a> {
    lines: Lines<'a>,
    buf: [Option<&'a str>; 2],
}

impl<'a> Iterator for Windows<'a> {
    type Item = LineWindow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let window = (self.buf[0].take(), self.buf[1].take()?, self.lines.next());
        self.buf = [Some(window.1), window.2];
        Some(window)
    }
}

pub fn windows(input: &str) -> Windows {
    let mut lines = input.lines();
    let buf = [None, lines.next()];
    Windows { lines, buf }
}

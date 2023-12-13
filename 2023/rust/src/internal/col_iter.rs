use std::str::Chars;

pub struct ColIter<'a> {
    lines: Vec<Chars<'a>>,
}

impl<'a> ColIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().map(|l| l.chars()).collect(),
        }
    }
}

impl<'a> Iterator for ColIter<'a> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: Vec<char> = Vec::new();
        for line in &mut self.lines {
            if let Some(char) = line.next() {
                res.push(char);
            } else {
                return None;
            }
        }
        Some(res)
    }
}

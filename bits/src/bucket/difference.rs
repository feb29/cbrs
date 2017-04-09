use std::iter::Peekable;
use std::cmp::Ordering::{Less, Equal, Greater};
use super::Iter;
use super::iterutil;

pub struct Difference<'a> {
    x: Peekable<Iter<'a>>,
    y: Peekable<Iter<'a>>,
}
impl<'a> Difference<'a> {
    pub fn new(x: Iter<'a>, y: Iter<'a>) -> Difference<'a> {
        Difference {
            x: x.peekable(),
            y: y.peekable(),
        }
    }
}

impl<'a> Iterator for Difference<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match iterutil::comparing(self.x.peek(), self.y.peek(), Less, Less) {
                Less => return self.x.next(),
                Equal => {
                    self.x.next();
                    self.y.next();
                }
                Greater => {
                    self.y.next();
                }
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let x_len = self.x.len();
        let y_len = self.y.len();
        (x_len.saturating_sub(y_len), Some(x_len))
    }
}

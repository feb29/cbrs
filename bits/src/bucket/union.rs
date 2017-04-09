use std::iter::Peekable;
use std::cmp;
use std::cmp::Ordering::{Less, Equal, Greater};
use super::Iter;
use super::iterutil;

pub struct Union<'a> {
    x: Peekable<Iter<'a>>,
    y: Peekable<Iter<'a>>,
}
impl<'a> Union<'a> {
    pub fn new(x: Iter<'a>, y: Iter<'a>) -> Union<'a> {
        Union {
            x: x.peekable(),
            y: y.peekable(),
        }
    }
}

impl<'a> Iterator for Union<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match iterutil::comparing(self.x.peek(), self.y.peek(), Greater, Less) {
                Less => return self.x.next(),
                Equal => {
                    self.y.next();
                    return self.x.next();
                }
                Greater => return self.y.next(),
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let x_len = self.x.len();
        let y_len = self.y.len();
        (cmp::max(x_len, y_len), Some(x_len + y_len))
    }
}

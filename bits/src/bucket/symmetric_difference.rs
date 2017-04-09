use std::iter::Peekable;
use std::cmp::Ordering::{Less, Equal, Greater};
use super::Iter;
use super::iterutil;

pub struct SymmetricDifference<'a> {
    x: Peekable<Iter<'a>>,
    y: Peekable<Iter<'a>>,
}
impl<'a> SymmetricDifference<'a> {
    pub fn new(x: Iter<'a>, y: Iter<'a>) -> SymmetricDifference<'a> {
        SymmetricDifference {
            x: x.peekable(),
            y: y.peekable(),
        }
    }
}

impl<'a> Iterator for SymmetricDifference<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match iterutil::comparing(self.x.peek(), self.y.peek(), Greater, Less) {
                Less => return self.x.next(),
                Equal => {
                    self.x.next();
                    self.y.next();
                }
                Greater => return self.y.next(),
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.x.len() + self.y.len()))
    }
}

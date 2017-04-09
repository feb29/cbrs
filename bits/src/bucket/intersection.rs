use std::iter::Peekable;
use std::cmp;
use std::cmp::Ordering::{Less, Equal, Greater};
use super::Iter;

pub struct Intersection<'a> {
    x: Peekable<Iter<'a>>,
    y: Peekable<Iter<'a>>,
}
impl<'a> Intersection<'a> {
    pub fn new(x: Iter<'a>, y: Iter<'a>) -> Intersection<'a> {
        Intersection {
            x: x.peekable(),
            y: y.peekable(),
        }
    }
}

impl<'a> Iterator for Intersection<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match {
                      let px = self.x.peek();
                      let py = self.y.peek();
                      px.and_then(|x1| py.map(|y1| x1.cmp(&y1)))
                  } {
                None => return None,
                Some(Less) => {
                    self.x.next();
                }
                Some(Equal) => {
                    self.y.next();
                    return self.x.next();
                }
                Some(Greater) => {
                    self.y.next();
                }
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let x_len = self.x.len();
        let y_len = self.y.len();
        (0, Some(cmp::min(x_len, y_len)))
    }
}

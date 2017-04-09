use std::iter::{Iterator, DoubleEndedIterator, ExactSizeIterator};
use std::{fmt, slice};

use super::{Bits, Bucket};

/// module document.

pub enum Iter<'a> {
    Vec {
        ones: usize,
        iter: slice::Iter<'a, u16>,
    },
    Map {
        // count of non-zero bit
        ones: usize,
        // iteration finished
        done: bool,

        // forward and reverse iterator.
        // assume ptr always points to valid bit or done.
        fwd: Ptr<'a>,
        rev: Ptr<'a>,
    },
}
impl<'a> fmt::Debug for Iter<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Iter::Vec { ref iter, .. } => write!(fmt, "Vec({:?})", iter),
            &Iter::Map {
                 ref done,
                 ref fwd,
                 ref rev,
                 ..
             } => write!(fmt, "Map({:?}, forward:{:?}, reverse:{:?})", done, fwd, rev),
        }
    }
}

impl<'a> Iter<'a> {
    pub fn vec(data: &'a [u16]) -> Iter<'a> {
        let ones = data.len();
        let iter = data.iter();
        Iter::Vec { ones, iter }
    }
    pub fn map(data: &'a [u64], ones: usize) -> Iter<'a> {
        let fwd = forward(data);
        let rev = reverse(data);
        let done = false;
        Iter::Map {
            ones,
            done,
            fwd,
            rev,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Iter::Vec { ref mut iter, .. } => iter.next().map(|&v| v),

            &mut Iter::Map { done: true, .. } => None,

            &mut Iter::Map {
                     ref mut fwd,
                     ref rev,
                     ref mut done,
                     ..
                 } => {
                let next = fwd.next();
                if fwd == rev {
                    *done = true;
                    return None;
                }
                return next;
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            &Iter::Vec { ref iter, .. } => iter.size_hint(),

            &Iter::Map { ones, .. } => (ones, Some(ones)),
        }
    }
}
impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            &mut Iter::Vec { ref mut iter, .. } => iter.next_back().map(|&v| v),

            &mut Iter::Map { done: true, .. } => None,

            &mut Iter::Map {
                     ref fwd,
                     ref mut rev,
                     ref mut done,
                     ..
                 } => {
                let back = rev.back();
                if fwd == rev {
                    *done = true;
                    return None;
                }
                return back;
            }
        }
    }
}
impl<'a> ExactSizeIterator for Iter<'a> {}

#[derive(Clone)]
pub struct Ptr<'a> {
    bits: &'a [u64],
    idx: usize, // current index
    pos: usize, // current position
    // If None, this pointer DO NOT point bit; initialized or reset
    bit: Option<u16>,
}

impl<'a> fmt::Debug for Ptr<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt,
               "Ptr(i:{}, p:{}, bit:{:?})",
               self.idx,
               self.pos,
               self.bit)
    }
}
impl<'a> PartialEq for Ptr<'a> {
    fn eq(&self, that: &Ptr<'a>) -> bool {
        let same_slice = self.bits.as_ptr() == that.bits.as_ptr();
        let same_point = self.idx == that.idx && self.pos == that.pos;
        let both_valid = self.bit.is_some() && that.bit.is_some() && self.bit == that.bit;
        same_slice && same_point && both_valid
    }
}

fn forward<'a>(bits: &'a [u64]) -> Ptr<'a> {
    Ptr {
        bits,
        idx: 0,
        pos: 0,
        bit: None,
    }
}
fn reverse<'a>(bits: &'a [u64]) -> Ptr<'a> {
    Ptr {
        bits,
        idx: bits.len() - 1,
        pos: Bucket::BITS_SIZE - 1,
        bit: None,
    }
}

impl<'a> Ptr<'a> {
    fn reset(&mut self, p: usize) {
        self.pos = p;
        self.bit = None;
    }

    // Lookup a next non-zero bit in `self.bits`.
    // Otherwise, nothing happen. Ptr points to same `idx` and `pos`.
    fn next(&mut self) -> Option<u16> {
        let idx = self.idx;
        let pos = self.pos;
        let bit = self.bit;

        if bit.is_some() {
            self.pos += 1;
        } else {
            // if bit is None, pos should be zero.
            debug_assert!(self.pos == 0);
        }

        if self.nextpos() {
            debug_assert!(self.bit.is_some(), "nextpos(): none bit");
            debug_assert!(bit != self.bit, "nextpos(): same bit");
            return self.bit;
        }

        self.reset(0); // no more non-zero bit in bits[i]

        while self.idx + 1 < self.bits.len() {
            self.idx += 1;
            if self.bits[self.idx] == 0 {
                // no non-zero bit
                continue;
            }
            let ok = self.nextpos();
            debug_assert!(ok);
            return self.bit;
        }

        self.idx = idx;
        self.pos = pos;
        self.bit = bit;
        return None;
    }
    // Lookup a non-zero bit in `self.bits[self.idx]` from `self.pos`.
    // If ptr already points to a non-zero bit, return true.
    // If lookup failed, nothing happen.
    fn nextpos(&mut self) -> bool {
        if self.pos >= Bucket::BITS_SIZE {
            return false; // need to reset
        }
        if let Some(&v) = self.bits.get(self.idx) {
            let i = self.idx;
            let mut p = self.pos;
            while p < Bucket::BITS_SIZE {
                if let some @ Some(_) = Ptr::checkbit(i, p, v) {
                    self.bit = some;
                    self.pos = p;
                    return true;
                }
                p += 1;
            }
        }
        return false;
    }

    fn back(&mut self) -> Option<u16> {
        let idx = self.idx;
        let pos = self.pos;
        let bit = self.bit;

        if self.pos >= 1 && bit.is_some() {
            self.pos -= 1;
        }
        if self.backpos() {
            debug_assert!(self.bit.is_some());
            return self.bit;
        }

        self.reset(Bucket::BITS_SIZE - 1);

        while self.idx >= 1 {
            self.idx -= 1;
            if self.bits[self.idx].count_ones() == 0 {
                continue;
            }
            let ok = self.backpos();
            debug_assert!(ok);
            return self.bit;
        }

        self.idx = idx;
        self.pos = pos;
        self.bit = bit;
        return None;
    }
    fn backpos(&mut self) -> bool {
        if self.pos == 0 && self.bit.is_some() {
            return false;
        }
        if let Some(&v) = self.bits.get(self.idx) {
            let i = self.idx;
            let mut p = self.pos;
            loop {
                if let some @ Some(_) = Ptr::checkbit(i, p, v) {
                    self.bit = some;
                    self.pos = p;
                    return true;
                }
                if p == 0 {
                    break;
                }
                p -= 1;
            }
        }
        return false;
    }

    fn checkbit(i: usize, p: usize, v: u64) -> Option<u16> {
        if v & (1 << p) != 0 {
            let bit64 = Bucket::BITS_SIZE * i + p;
            debug_assert!(bit64 < Bucket::SIZE);
            let bit16 = bit64 as u16;
            return Some(bit16);
        }
        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_partialeq() {
        let vec_0 = vec![0];
        let vec_1 = vec![0];

        let mut p0 = Ptr {
            bits: &vec_0[..],
            idx: 0,
            pos: 0,
            bit: None,
        };

        let mut p1 = Ptr {
            bits: &vec_0[..],
            idx: 0,
            pos: 0,
            bit: None,
        };

        assert_ne!(p0, p1, "invalid ptr should not be equal");
        p1.bits = &vec_1[..];
        assert_ne!(p0, p1, "slice raw pointer");

        p0.bits = &vec_1[..];
        p0.bit = Some(0);
        p1.bit = Some(0);
        assert_eq!(p0, p1, "slice raw pointer");
    }
}

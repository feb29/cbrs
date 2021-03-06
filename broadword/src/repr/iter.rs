use std::iter::{Iterator, ExactSizeIterator};
use std::slice::Iter as SliceIter;
use std::marker::PhantomData;

use super::{Bits, Repr};

/// module document.

// each 'ones' are count of non-zero bit; for size_hint
pub enum Iter<'a> {
    Vec {
        ones: usize,
        iter: SliceIter<'a, u16>,
    },
    Map {
        ones: usize,
        ptr: SlicePtr<'a, Forward>,
    },
}

impl<'a> Iter<'a> {
    pub fn vec(bits: &'a [u16], ones: usize) -> Iter<'a> {
        debug_assert!(bits.len() == ones);
        debug_assert!(ones <= Repr::SIZE, "{:?} {:?}", ones, Repr::SIZE);
        let iter = bits.iter();
        Iter::Vec { ones, iter }
    }
    pub fn map(bits: &'a [u64], ones: usize) -> Iter<'a> {
        debug_assert!(ones <= Repr::SIZE);
        let ptr = SlicePtr::new_forward(bits);
        Iter::Map { ones, ptr }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut Iter::Vec { ref mut iter, .. } => iter.next().cloned(),
            &mut Iter::Map { ref mut ptr, .. } => ptr.forward(),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            &Iter::Vec { ref iter, .. } => iter.size_hint(),
            &Iter::Map { ones, .. } => (ones, Some(ones)),
        }
    }
}
impl<'a> ExactSizeIterator for Iter<'a> {
    /*
    fn is_empty(&self) -> bool {
        match self {
            &Iter::Vec { ref ones, .. } => ones == 0,
            &Iter::Map { ref ones, .. } => ones == 0,
        }
    }
    */
}

pub struct Forward;

pub struct SlicePtr<'a, T> {
    bits: &'a [u64],
    idx: usize,
    pos: usize,
    _dir: PhantomData<T>,
}

impl<'a> SlicePtr<'a, Forward> {
    fn new_forward(bits: &'a [u64]) -> Self {
        SlicePtr {
            bits,
            idx: 0,
            pos: 0,
            _dir: PhantomData,
        }
    }
    fn prepare(&mut self) {
        self.pos += 1;
        if self.pos == Repr::BITS_SIZE {
            self.pos = 0;
            self.idx += 1;
        }
    }
}

impl<'a> SlicePtr<'a, Forward> {
    fn forward(&mut self) -> Option<u16> {
        loop {
            let i = self.idx;
            let p = self.pos;
            if i >= self.bits.len() {
                return None;
            } else if self.bits[i] & (1u64 << p) != 0 {
                let bit = Some((i * Repr::BITS_SIZE + p) as u16);
                self.prepare();
                return bit;
            }
            self.prepare();
        }
    }
}

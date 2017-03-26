#![feature(test)]
extern crate test;

use std::ops;
use std::num::Wrapping;

mod broadword;

#[derive(Debug, Copy, Clone)]
struct Bits(u64);

impl Bits {
    fn pop_count(&self) -> usize {
        self.0.count_ones() as usize
    }
    fn rank1(&self, i: usize) -> usize {
        broadword::rank9(self.0, i)
    }
    fn rank0(&self, i: usize) -> usize {
        broadword::rank9(!self.0, i)
    }
    fn select1(&self, c: usize) -> usize {
        broadword::select9(self.0, c)
    }
    fn select0(&self, c: usize) -> usize {
        broadword::select9(!self.0, c)
    }
}

macro_rules! bits_shift_impl {
    ( $($shift: ty),* ) => ($(
        impl ops::Shr<$shift> for Bits {
            type Output = Self;
            fn shr(self, shift: $shift) -> Self::Output {
                let Bits(lhs) = self;
                let Wrapping(r) = Wrapping(lhs) >> shift as usize;
                Bits(r)
            }
        }
        impl ops::Shl<$shift> for Bits {
            type Output = Self;
            fn shl(self, shift: $shift) -> Self::Output {
                let Bits(lhs) = self;
                let Wrapping(r) = Wrapping(lhs) << shift as usize;
                Bits(r)
            }
        }
    )*)
}

macro_rules! bits_shift_assign_impl {
    ( $($shift: ty),* ) => ($(
        impl ops::ShrAssign<$shift> for Bits {
            fn shr_assign(&mut self, shift: $shift) { *self = self.clone() >> shift }
        }
        impl ops::ShlAssign<$shift> for Bits {
            fn shl_assign(&mut self, shift: $shift) { *self = self.clone() << shift }
        }
    )*)
}

bits_shift_impl!(u16, u32, u64, usize);
bits_shift_assign_impl!(u16, u32, u64, usize);

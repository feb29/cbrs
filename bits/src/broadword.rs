//! Broadword implementation of rank/select queries;
//! Springer Berlin Heidelberg, 2008. 154-168.

#![allow(dead_code)]

const X01: u64 = 0x0101010101010101;
const X02: u64 = 0x2020202020202020;
const X33: u64 = 0x3333333333333333;
const X22: u64 = 0x2222222222222222;
const X80: u64 = 0x2010080402010080;
const X81: u64 = 0x2010080402010081;
const X0F: u64 = 0x0f0f0f0f0f0f0f0f;
const X55: u64 = X22 + X33 + X22 + X33;
const X8X: u64 = X81 + X80 + X80 + X80;

use super::Bits;

pub trait Rank<T = usize>: Bits {
    /// Count how many non-zero bits there are up to a given position
    fn rank1(&self, i: usize) -> T;
    fn rank0(&self, i: usize) -> T;
}
pub trait Select<T = usize>: Bits {
    /// Return the 'c+1'th non-zero bit's index.
    fn select1(&self, i: usize) -> Option<T>;
    fn select0(&self, i: usize) -> Option<T>;
}

macro_rules! impl_rank9 {
    ( $( ($type: ty, $out: ty) ),* ) => ($(
        impl Rank<$out> for $type {
            #[inline]
            fn rank1(&self, i: usize) -> $out {
                let rank = if i >= Self::SIZE {
                    self.ones()
                } else {
                    let this = *self;
                    (this & ((1 << i) - 1)).ones()
                };
                rank as $out
            }
            #[inline]
            fn rank0(&self, i: usize) -> $out {
                let rank1: $out = self.rank1(i);
                i as $out - rank1
            }
        }
    )*)
}
macro_rules! impl_rank9_all {
    ( $( $type: ty ),* ) => ($(
        impl_rank9!(($type, u64), ($type, u32), ($type, u16), ($type, u8), ($type, usize));
    )*)
}
impl_rank9_all!(u64, u32, u16, u8, usize);

macro_rules! impl_select9 {
    ( $( ($type: ty, $out: ty) ),* ) => ($(
        impl Select<$out> for $type {
            #[inline]
            fn select1(&self, c: usize) -> Option<$out> {
                let x = *self as u64;
                let s0 = x - ((x & X55) >> 1);
                let s1 = (s0 & X33) + ((s0 >> 2) & X33);
                let s2 = ((s1 + (s1 >> 4)) & X0F).wrapping_mul(X01);
                let p0 = (le8(s2, (c as u64 * X01)) >> 7).wrapping_mul(X01);
                let p1 = (p0 >> 53) & !0x7;
                let p2 = p1 as u32;
                let p3 = (s2 << 8).wrapping_shr(p2);
                let p4 = c - (p3 & 0xFF) as usize;
                let p5 = lt8(0x0, ((x.wrapping_shr(p2) & 0xFF) * X01) & X8X);
                let s3 = (p5 >> 0x7).wrapping_mul(X01);
                let p6 = (le8(s3, (p4 as u64 * X01)) >> 7).wrapping_mul(X01) >> 56;
                let p = p1 + p6;
                if p >= Self::SIZE as u64 { None } else { Some(p as $out) }
            }
            #[inline]
            fn select0(&self, c: usize) -> Option<$out> { (!*self).select1(c) }
        }
    )*)
}
macro_rules! impl_select9_all {
    ( $( $type: ty ),* ) => ($(
        impl_select9!(($type, u64), ($type, u32), ($type, u16), ($type, u8), ($type, usize));
    )*)
}
impl_select9_all!(u64, u32, u16, u8, usize);

fn le8(x: u64, y: u64) -> u64 {
    let x8 = X02 + X02 + X02 + X02;
    let xs = (y | x8) - (x & !x8);
    (xs ^ x ^ y) & x8
}

fn lt8(x: u64, y: u64) -> u64 {
    let x8 = X02 + X02 + X02 + X02;
    let xs = (x | x8) - (y & !x8);
    (xs ^ x ^ !y) & x8
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test(u64, (usize, Option<usize>));

    static TESTS: &[Test] = &[Test(0b_0000000000_0000000000, (0, None)),
                              Test(0b_0000100101_1000111001, (1, Some(3))),
                              Test(0b_0000100101_1000111001, (2, Some(4))),
                              Test(0b_0000100101_1000111001, (3, Some(5))),
                              Test(0b_0000100101_1000111001, (4, Some(9))),
                              Test(0b_0000100101_1000111001, (5, Some(10))),
                              Test(0b_0000100101_1000111001, (6, Some(12))),
                              Test(0b_0000100101_1000111001, (7, Some(15))),
                              Test(0b_0000100101_0000000000, (0, Some(10))),
                              Test(0b_0000100101_0000000000, (1, Some(12))),
                              Test(0b_0000100101_0000000000, (2, Some(15))),
                              Test(0b_0000000000_0000000001, (0, Some(0))),
                              Test(0b_0000100101_0000000000, (3, None)),
                              Test(0b_0000000000_0000000001, (1, None))];

    #[test]
    fn broadword_properties() {
        for &Test(bits, (k, want)) in TESTS {
            assert_eq!(bits.select1(k), want);

            let r9: u32 = bits.rank1(64);
            assert_eq!(r9, bits.count_ones());

            let s9: Option<usize> = bits.select1(k);
            if let Some(s) = s9 {
                let r9: usize = bits.rank1(s);
                assert_eq!(r9, k);
            }
        }
    }
}

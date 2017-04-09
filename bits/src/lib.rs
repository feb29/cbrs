#![feature(associated_consts)]
#![feature(test)]

extern crate test;

mod broadword;
use broadword::{Rank, Select};

mod bucket;

// Constant sized bits.
pub trait Bits {
    const SIZE: usize;

    fn zero() -> Self;

    // Count non-zero bits.
    // REQUIRES: ones() <= SIZE
    fn ones(&self) -> usize;
}
macro_rules! impl_sizedbits {
    ( $( ($type: ty, $size: expr) ),* ) => ($(
        impl Bits for $type {
            const SIZE: usize = $size;
            fn zero() -> Self { 0 }
            fn ones(&self) -> usize {
                let ones = self.count_ones();
                debug_assert!(ones as usize <= Self::SIZE);
                ones as usize
            }
        }
    )*)
}
impl_sizedbits!((u64, 64), (u32, 32), (u16, 16), (u8, 8));
#[cfg(target_pointer_width = "32")]
impl_sizedbits!{(usize, 32)}
#[cfg(target_pointer_width = "64")]
impl_sizedbits!{(usize, 64)}

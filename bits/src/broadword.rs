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

const SIZE: usize = 64;

// Broadword implementation of rank/select queries;
// Springer Berlin Heidelberg, 2008. 154-168.
// `rank` count how many bits there are up to a given position

#[inline]
pub fn rank9(bits: u64, i: usize) -> usize {
    if i >= SIZE {
        bits.count_ones() as usize
    } else {
        (bits & ((1 << i) - 1)).count_ones() as usize
    }
}

/// Return the 'c+1'th non-zero bit's index, or return 72.
#[inline]
pub fn select9(x: u64, c: usize) -> usize {
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
    (p1 + p6) as usize
}

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

#[inline]
fn select_search(x: u64, c: usize) -> usize {
    debug_assert!(x.count_ones() as usize >= c);
    let mut i = 0;
    let mut j = SIZE;
    while i < j {
        let h = i + (j - i) / 2;
        if rank9(x, h) <= c {
            i = h + 1;
        } else {
            j = h;
        }
    }
    i - 1
}

#[cfg(test)]
mod tests {
    use super::{rank9, select9};

    struct Test(u64, (usize, usize));

    static TESTS: &[Test] = &[Test(0b_0000000000_0000000000, (0, 72)),
                              Test(0b_0000100101_1000111001, (1, 3)),
                              Test(0b_0000100101_1000111001, (2, 4)),
                              Test(0b_0000100101_1000111001, (3, 5)),
                              Test(0b_0000100101_1000111001, (4, 9)),
                              Test(0b_0000100101_1000111001, (5, 10)),
                              Test(0b_0000100101_1000111001, (6, 12)),
                              Test(0b_0000100101_1000111001, (7, 15)),
                              Test(0b_0000100101_0000000000, (0, 10)),
                              Test(0b_0000100101_0000000000, (1, 12)),
                              Test(0b_0000100101_0000000000, (2, 15)),
                              Test(0b_0000100101_0000000000, (3, 72)),
                              Test(0b_0000000000_0000000001, (0, 0)),
                              Test(0b_0000000000_0000000001, (1, 72))];

    #[test]
    fn broadword_properties() {
        for &Test(bits, (k, want)) in TESTS {
            assert_eq!(select9(bits, k), want);
            assert_eq!(rank9(bits, 64), bits.count_ones() as usize);
            assert_eq!(rank9(bits, select9(bits, k)), k);
        }
    }
}

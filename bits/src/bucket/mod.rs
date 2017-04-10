use std::{fmt, ops, u16};
use std::iter::{FromIterator, IntoIterator};

use super::{Bits, Rank, Select};

mod repr;
use self::repr::Repr;

mod iter;
pub use self::iter::Iter;
mod iterutil;

mod intersection;
pub use self::intersection::Intersection;
mod union;
pub use self::union::Union;
mod difference;
pub use self::difference::Difference;
mod symmetric_difference;
pub use self::symmetric_difference::SymmetricDifference;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Bucket {
    popc: usize,
    repr: Repr,
}
impl Bits for Bucket {
    const SIZE: usize = u16::MAX as usize;

    fn zero() -> Self {
        let popc = 0;
        let repr = Repr::new();
        Bucket { popc, repr }
    }
    fn ones(&self) -> usize {
        self.popc
    }
}
impl Bucket {
    fn iter(&self) -> Iter {
        match self.repr {
            Repr::Vec(ref bits) => Iter::vec(&bits[..]),
            Repr::Map(ref bits) => Iter::map(&bits[..], self.popc),
        }
    }
}

impl Bucket {
    const BITS_SIZE: usize = <u64 as Bits>::SIZE;
    const THRESHOLD: usize = Repr::VEC_SIZE / Self::SIZE;

    fn load_factor(&self) -> f64 {
        self.ones() as f64 / Self::SIZE as f64
    }

    pub fn contains(&self, bit: u16) -> bool {
        self.repr.contains(bit)
    }
    pub fn insert(&mut self, bit: u16) -> bool {
        let ok = self.repr.insert(bit);
        if ok {
            self.popc += 1;
        }
        ok
    }
    pub fn remove(&mut self, bit: u16) -> bool {
        let ok = self.repr.remove(bit);
        if ok {
            self.popc -= 1;
        }
        ok
    }

    /// Convert to more size efficient bits representaions.
    pub fn optimize(&mut self) {
        if self.repr.fitted(self.popc) {
            return;
        }

        let popc = self.popc;
        let mut repr = Repr::with_capacity(self.popc);

        for x in self.iter() {
            assert!(repr.insert(x));
        }
        *self = Bucket { popc, repr };
    }
}


impl fmt::Debug for Bucket {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt,
               "Bucket::{repr:?}{{{load_factor:.5}}}",
               repr = self.repr,
               load_factor = self.load_factor())
    }
}

impl Rank for Bucket {
    fn rank1(&self, i: usize) -> usize {
        if i >= Bucket::SIZE {
            return self.ones();
        }
        let rank = match self.repr {
            Repr::Vec(ref bits) => {
                let j = i as u16;
                match bits.binary_search(&j) {
                    Err(r) if r > bits.len() => self.ones(), // rank - 1
                    Err(r) | Ok(r) => r,
                }
            }
            Repr::Map(ref bits) => {
                let q = i / Bucket::BITS_SIZE;
                let r = i % Bucket::BITS_SIZE;
                bits.iter().take(q).fold(0, |acc, w| acc + w.ones()) +
                bits.get(q).map_or(0, |w| w.rank1(r))
            }
        };
        return rank;
    }
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }
}

impl Select for Bucket {
    fn select1(&self, c: usize) -> Option<usize> {
        if c >= self.ones() {
            return None;
        }
        match self.repr {
            Repr::Vec(ref bits) => bits.get(c).map(|&u| u as usize),
            Repr::Map(ref bits) => {
                let mut r = c;
                for (i, x) in bits.iter().enumerate() {
                    let w = x.ones();
                    if r < w {
                        return Some(Self::BITS_SIZE * i + x.select1(r).unwrap_or(0));
                    }
                    r -= w;
                }
                None
            }
        }
    }
    fn select0(&self, c: usize) -> Option<usize> {
        unimplemented!()
    }
}

impl FromIterator<u16> for Bucket {
    fn from_iter<I: IntoIterator<Item = u16>>(iterable: I) -> Bucket {
        let iter = iterable.into_iter();
        let size = Repr::VEC_SIZE;
        match iter.size_hint() {
            (_, Some(max)) if max <= size => {
                let vec = iter.collect::<Vec<u16>>();
                let popc = vec.len();
                let repr = Repr::Vec(vec);
                Bucket { popc, repr }
            }

            (min, maxopt) => {
                let cmp = maxopt.map_or(min, |max| max);
                let mut repr = Repr::with_capacity(cmp);
                let popc = insert_u16_all(iter, &mut repr);
                let mut bucket = Bucket { popc, repr };
                bucket.optimize();
                bucket
            }
        }
    }
}
impl<'a> FromIterator<&'a u16> for Bucket {
    fn from_iter<I: IntoIterator<Item = &'a u16>>(iterable: I) -> Bucket {
        let iter = iterable.into_iter();
        iter.map(|&v| v).collect::<Bucket>()
    }
}

impl FromIterator<bool> for Bucket {
    fn from_iter<I: IntoIterator<Item = bool>>(iterable: I) -> Bucket {
        let iter = iterable.into_iter();
        iter.take(Bucket::SIZE)
            .enumerate()
            .filter_map(|(i, p)| if p { Some(i as u16) } else { None })
            .collect::<Bucket>()
    }
}
impl<'a> FromIterator<&'a bool> for Bucket {
    fn from_iter<I: IntoIterator<Item = &'a bool>>(iterable: I) -> Bucket {
        let iter = iterable.into_iter();
        iter.map(|&v| v).collect::<Bucket>()
    }
}

fn insert_u16_all<It: Iterator<Item = u16>>(it: It, repr: &mut Repr) -> usize {
    let mut popc = 0;
    for item in it {
        if repr.insert(item) {
            popc += 1;
        }
    }
    popc
}

impl<'a> ops::BitAnd<&'a Bucket> for &'a Bucket {
    type Output = &'a Bucket;
    #[allow(unused_variables)]
    fn bitand(self, that: &Bucket) -> Self::Output {
        unimplemented!();
    }
}
impl<'a> ops::BitAndAssign<&'a Bucket> for Bucket {
    #[allow(unused_variables)]
    fn bitand_assign(&mut self, that: &Bucket) {
        unimplemented!();
    }
}

impl<'a> ops::BitOr<&'a Bucket> for &'a Bucket {
    type Output = &'a Bucket;
    #[allow(unused_variables)]
    fn bitor(self, that: &Bucket) -> Self::Output {
        unimplemented!();
    }
}
impl<'a> ops::BitOrAssign<&'a Bucket> for Bucket {
    #[allow(unused_variables)]
    fn bitor_assign(&mut self, that: &Bucket) {
        unimplemented!();
    }
}

impl<'a> ops::BitXor<&'a Bucket> for &'a Bucket {
    type Output = &'a Bucket;
    #[allow(unused_variables)]
    fn bitxor(self, that: &Bucket) -> Self::Output {
        unimplemented!();
    }
}
impl<'a> ops::BitXorAssign<&'a Bucket> for Bucket {
    #[allow(unused_variables)]
    fn bitxor_assign(&mut self, that: &Bucket) {
        unimplemented!();
    }
}

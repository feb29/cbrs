use std::fmt;
use super::{Bits, Bucket};

#[derive(Clone)]
pub enum Repr {
    // Vec hold bit as is, sorted order.
    Vec(Vec<u16>),
    // Map hold u64 as a bitarray, each non-zero bit represents element.
    Map(Vec<u64>),
}
impl fmt::Debug for Repr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Repr::Vec(_) => write!(fmt, "Vec"),
            &Repr::Map(_) => write!(fmt, "Map"),
        }
    }
}

macro_rules! bitmask {
    ( $bit: expr, $index: ident, $mask: ident ) => (
        let $index = $bit as usize / Bucket::BITS_SIZE;
        let shift  = $bit % Bucket::BITS_SIZE as u16;
        let $mask  = 1 << shift;
    );
}

impl Repr {
    pub const VEC_SIZE: usize = 4096;
    pub const MAP_SIZE: usize = Bucket::SIZE / Bucket::BITS_SIZE + 1;

    pub fn fitted(&mut self, ones: usize) -> bool {
        match self {
            &mut Repr::Vec(_) if ones > Self::VEC_SIZE => false,
            &mut Repr::Map(_) if ones <= Self::VEC_SIZE => false,

            &mut Repr::Vec(ref mut bits) => {
                bits.shrink_to_fit();
                return true;
            }
            &mut Repr::Map(ref mut bits) => {
                bits.shrink_to_fit();
                return true;
            }
        }
    }
}

impl Repr {
    pub fn new() -> Repr {
        Repr::Vec(Vec::new())
    }

    pub fn with_capacity(cap: usize) -> Repr {
        if cap <= Self::VEC_SIZE {
            let vec = Vec::with_capacity(cap);
            Repr::Vec(vec)
        } else {
            let vec = Vec::with_capacity(cap);
            Repr::Map(vec)
        }
    }

    pub fn is_vec(&self) -> bool {
        match self {
            &Repr::Vec(_) => true,
            _ => false,
        }
    }
    pub fn is_map(&self) -> bool {
        match self {
            &Repr::Map(_) => true,
            _ => false,
        }
    }

    pub fn contains(&self, bit: u16) -> bool {
        match self {
            &Repr::Vec(ref bits) => bits.binary_search(&bit).is_ok(),
            &Repr::Map(ref bits) => {
                bitmask!(bit, i, mask);
                bits.get(i).map_or(false, |map| *map & mask != 0)
            }
        }
    }

    pub fn insert(&mut self, bit: u16) -> bool {
        match self {
            &mut Repr::Vec(ref mut bits) => {
                bits.binary_search(&bit)
                    .map_err(|i| bits.insert(i, bit))
                    .is_err()
            }
            &mut Repr::Map(ref mut bits) => {
                bitmask!(bit, i, mask);
                if let Some(map) = bits.get_mut(i) {
                    if *map & mask != 0 {
                        return false;
                    } else {
                        *map |= mask;
                        return true;
                    }
                }
                if i > bits.len() {
                    bits.resize(i, 0);
                }
                bits.insert(i, mask);
                return true;
            }
        }
    }

    pub fn remove(&mut self, bit: u16) -> bool {
        match self {
            &mut Repr::Vec(ref mut bits) => {
                bits.binary_search(&bit)
                    .map(|i| {
                             let removed = bits.remove(i);
                             debug_assert_eq!(bit, removed);
                         })
                    .is_ok()
            }
            &mut Repr::Map(ref mut bits) => {
                bitmask!(bit, i, mask);
                if let Some(map) = bits.get_mut(i) {
                    if *map & mask != 0 {
                        *map &= !mask;
                        return true;
                    } else {
                        return false;
                    };
                }
                return false;
            }
        }
    }
}

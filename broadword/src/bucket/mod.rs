use super::repr::Repr;

//mod intersection;
//pub use self::intersection::Intersection;
//mod union;
//pub use self::union::Union;
//mod difference;
//pub use self::difference::Difference;
//mod symmetric_difference;
//pub use self::symmetric_difference::SymmetricDifference;

// #[cfg(test)] mod bucket_test;

#[derive(Clone)]
pub struct Bucket {
    popc: usize,
    repr: Repr,
}

/*
impl Bucket {
    fn iter(&self) -> Iter {
        match self.repr {
            Repr::Vec(ref bits) => Iter::vec(&bits[..]),
            Repr::Map(ref bits) => Iter::map(&bits[..], self.popc),
        }
    }
    fn intersection<'a>(&'a self, that: &'a Self) -> Intersection<'a> {
        Intersection::new(self.iter(), that.iter())
    }
    fn union<'a>(&'a self, that: &'a Self) -> Union<'a> {
        Union::new(self.iter(), that.iter())
    }
    fn difference<'a>(&'a self, that: &'a Self) -> Difference<'a> {
        Difference::new(self.iter(), that.iter())
    }
    fn symmetric_difference<'a>(&'a self, that: &'a Self) -> SymmetricDifference<'a> {
        SymmetricDifference::new(self.iter(), that.iter())
    }
}

impl Bucket {
    const BITS_SIZE: usize = <u64 as Bits>::SIZE;
    const THRESHOLD: usize = Repr::VEC_SIZE / Self::SIZE;

    fn new() -> Bucket {
        Bucket::none()
    }
    fn with_capacity(size: usize) -> Bucket {
        let repr = Repr::with_capacity(size);
        let popc = 0;
        Bucket { popc, repr }
    }

    fn load_factor(&self) -> f64 {
        self.ones() as f64 / Self::SIZE as f64
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

impl Bucket {
    fn contains(&self, item: u16) -> bool {
        self.repr.contains(item)
    }
    fn insert(&mut self, item: u16) -> bool {
        let ok = self.repr.insert(item);
        if ok {
            self.popc += 1;
        }
        ok
    }
    fn remove(&mut self, item: u16) -> bool {
        let ok = self.repr.remove(item);
        if ok {
            self.popc -= 1;
        }
        ok
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
*/

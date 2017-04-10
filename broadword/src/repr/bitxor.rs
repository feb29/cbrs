use std::ops;
use super::{pair, Bits, Repr};

macro_rules! symmetric_difference {
    ( $iter: ident, $vec0: expr, $vec1: expr ) => {
        let $iter = {
            let i0 = $vec0.iter();
            let i1 = $vec1.iter();
            pair::symmetric_difference(i0, i1)
        };
    };
}
macro_rules! clone_symmetric_difference {
    ( $clone: ident, $source: expr, $target: expr ) => {
        let mut $clone = $source.clone();
        $clone.symmetric_difference_with($target);
    };
}

impl Repr {
    fn symmetric_difference_with(&mut self, that: &Repr) {
        match (self, that) {
            (vec0 @ &mut Repr::Vec(..), vec1 @ &Repr::Vec(..)) => {
                let repr = vec0.clone();
                symmetric_difference!(iter, repr, vec1);
                *vec0 = iter.collect::<Repr>();
            }

            (this @ &mut Repr::Vec(..), &Repr::Map(..)) => {
                clone_symmetric_difference!(clone, that, this);
                *this = clone;
            }

            (ref mut this @ &mut Repr::Map(..), &Repr::Vec(_, ref bits)) => {
                for &bit in bits.iter() {
                    if this.contains(bit) {
                        this.remove(bit);
                    } else {
                        this.insert(bit);
                    }
                }
            }
            (&mut Repr::Map(ref mut ones, ref mut bits0), &Repr::Map(_, ref bits1)) => {
                *ones = 0;
                for (x, y) in bits0.iter_mut().zip(bits1.iter()) {
                    let p = *x ^ *y;
                    *ones += p.ones();
                    *x = p;
                }
            }
        }
    }
}

impl<'a, 'b> ops::BitXor<&'b Repr> for &'a Repr {
    type Output = Repr;
    fn bitxor(self, that: &Repr) -> Self::Output {
        match (self, that) {
            (vec0 @ &Repr::Vec(..), vec1 @ &Repr::Vec(..)) => {
                let repr = vec0.clone();
                symmetric_difference!(iter, repr, vec1);
                iter.collect::<Repr>()
            }
            (vec @ &Repr::Vec(..), map @ &Repr::Map(..)) => {
                clone_symmetric_difference!(clone, map, vec);
                clone
            }
            (this, that) => {
                clone_symmetric_difference!(clone, this, that);
                clone
            }
        }
    }
}
impl<'a> ops::BitXorAssign<&'a Repr> for Repr {
    fn bitxor_assign(&mut self, that: &Repr) {
        self.symmetric_difference_with(that);
    }
}

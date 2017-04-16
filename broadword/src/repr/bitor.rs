use std::ops;
use super::{pair, Bits, Repr};

macro_rules! union {
    ( $iter: ident, $vec0: expr, $vec1: expr ) => {
        let $iter = {
            let i0 = $vec0.iter();
            let i1 = $vec1.iter();
            pair::union(i0, i1)
        };
    };
}
macro_rules! clone_union_with {
    ( $clone: ident, $source: expr, $target: expr ) => {
        let mut $clone = $source.clone();
        $clone.union_with($target);
    };
}

impl Repr {
    fn union_with(&mut self, that: &Repr) {
        match (self, that) {
            (vec0 @ &mut Repr::Vec(..), vec1 @ &Repr::Vec(..)) => {
                let repr = vec0.clone();
                union!(iter, repr, vec1);
                *vec0 = iter.collect::<Repr>();
            }

            (ref mut repr @ &mut Repr::Map(..), &Repr::Vec(_, ref bits)) => {
                for &b in bits {
                    repr.insert(b);
                }
            }

            (mut repr @ &mut Repr::Vec(..), map_repr @ &Repr::Map(..)) => {
                clone_union_with!(clone, map_repr, repr);
                *repr = clone;
            }

            (&mut Repr::Map(ref mut ones, ref mut bits0), &Repr::Map(_, ref bits1)) => {
                *ones = 0;
                for (x, y) in bits0.iter_mut().zip(bits1.iter()) {
                    let p = *x | *y;
                    *ones += p.ones();
                    *x = p;
                }
            }
        }
    }
}

impl<'a, 'b> ops::BitOr<&'b Repr> for &'a Repr {
    type Output = Repr;
    fn bitor(self, that: &Repr) -> Self::Output {
        match (self, that) {
            (vec0 @ &Repr::Vec(..), vec1 @ &Repr::Vec(..)) => {
                let repr = vec0.clone();
                union!(iter, repr, vec1);
                iter.collect::<Repr>()
            }
            (vec @ &Repr::Vec(..), map @ &Repr::Map(..)) => {
                clone_union_with!(clone, map, vec);
                clone
            }
            (this, that) => {
                clone_union_with!(clone, this, that);
                clone
            }
        }
    }
}
impl<'a> ops::BitOrAssign<&'a Repr> for Repr {
    fn bitor_assign(&mut self, that: &Repr) {
        self.union_with(that);
    }
}

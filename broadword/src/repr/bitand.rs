use std::ops;
use super::{pair, Bits, Repr};

macro_rules! intersection {
    ( $iter: ident, $vec0: expr, $vec1: expr ) => {
        let $iter = {
            let i0 = $vec0.iter();
            let i1 = $vec1.iter();
            pair::intersection(i0, i1)
        };
    };
}
macro_rules! clone_intersect_with {
    ( $clone: ident, $source: expr, $target: expr ) => {
        let mut $clone = $source.clone();
        $clone.intersect_with($target);
    };
}

impl Repr {
    fn intersect_with(&mut self, that: &Repr) {
        match (self, that) {
            (vec0 @ &mut Repr::Vec(..), vec1 @ &Repr::Vec(..)) => {
                let repr = vec0.clone();
                intersection!(iter, repr, vec1);
                *vec0 = iter.collect::<Repr>();
            }

            (repr @ &mut Repr::Map(..), &Repr::Vec(..)) => {
                clone_intersect_with!(clone, that, repr);
                *repr = clone;
            }

            (&mut Repr::Vec(ref mut ones, ref mut bits0), map_repr @ &Repr::Map(..)) => {
                *ones = 0;
                for i in 0..bits0.len() {
                    if map_repr.contains(bits0[i]) {
                        bits0[*ones] = bits0[i];
                        *ones += 1;
                    }
                }
                bits0.truncate(*ones);
            }

            (&mut Repr::Map(ref mut ones, ref mut bits0), &Repr::Map(_, ref bits1)) => {
                *ones = 0;
                for (x, y) in bits0.iter_mut().zip(bits1.iter()) {
                    let p = *x & *y;
                    *ones += p.ones();
                    *x = p;
                }
            }
        }
    }
}

impl<'a, 'b> ops::BitAnd<&'b Repr> for &'a Repr {
    type Output = Repr;
    fn bitand(self, that: &Repr) -> Self::Output {
        match (self, that) {
            (vec0 @ &Repr::Vec(..), vec1 @ &Repr::Vec(..)) => {
                intersection!(iter, vec0, vec1);
                iter.collect::<Repr>()
            }
            (map @ &Repr::Map(..), vec @ &Repr::Vec(..)) => {
                clone_intersect_with!(clone, vec, map);
                clone
            }
            (this, that) => {
                clone_intersect_with!(clone, this, that);
                clone
            }
        }
    }
}
impl<'a> ops::BitAndAssign<&'a Repr> for Repr {
    fn bitand_assign(&mut self, that: &Repr) {
        self.intersect_with(that)
    }
}

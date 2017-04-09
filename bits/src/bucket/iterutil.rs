use std::cmp::Ordering;

/// Compare `a` and `b`, but return `s` if a is None and `l` if b is None
pub fn comparing<T: Ord>(a: Option<T>, b: Option<T>, x: Ordering, y: Ordering) -> Ordering {
    match (a, b) {
        (None, _) => x,
        (_, None) => y,
        (Some(ref a1), Some(ref b1)) => a1.cmp(b1),
    }
}

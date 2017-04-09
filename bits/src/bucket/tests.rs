extern crate env_logger;
extern crate rand;

use self::rand::Rng;
use super::*;

fn generate_bucket(size: usize) -> Bucket {
    let mut rng = rand::thread_rng();
    let mut b = Bucket::zero();
    for _ in 0..size {
        let x = rng.gen();
        b.insert(x);
    }
    return b;
}

#[test]
fn bucket_properties() {
    let mut rng = rand::thread_rng();
    {
        let l0 = rng.gen_range(10, Repr::VEC_SIZE + 1);
        let b0 = generate_bucket(l0);
        assert_eq!(b0.ones(), b0.rank1(Bucket::SIZE));

        let c = rng.gen_range(0, b0.ones() - 1);
        let s = b0.select1(c).unwrap();
        let r = b0.rank1(s);
        assert_eq!(c, r);
    }
    {
        let l1 = rng.gen_range(Repr::VEC_SIZE + 1, Bucket::SIZE - 1);
        let b1 = generate_bucket(l1);
        assert_eq!(b1.ones(), b1.rank1(Bucket::SIZE));

        let c = rng.gen_range(0, b1.ones() - 1);
        let s = b1.select1(c).unwrap();
        let r = b1.rank1(s);
        assert_eq!(c, r);
    }
}

#[test]
fn bucket_from_iter() {
    let _ = env_logger::init();
    {
        let vec = vec![1u16;10000];
        let bucket = vec.iter().collect::<Bucket>();
        debug!("{:?} {:?}", bucket, bucket.ones());
    }
    {
        let vec = vec![true;10000];
        let bucket = vec.iter().collect::<Bucket>();
        debug!("{:?} {:?}", bucket, bucket.ones());
    }
}

#[test]
fn bucket_insert_remove() {
    let _ = env_logger::init();

    let mut b = Bucket::zero();
    let mut i = 0u16;
    while i < Repr::VEC_SIZE as u16 {
        assert!(b.insert(i), format!("insert({:?}) failed", i));
        assert!(b.contains(i));
        i += 1;
    }
    assert_eq!(b.ones(), Repr::VEC_SIZE);
    while i < Bucket::SIZE as u16 {
        assert!(b.insert(i), "insert failed");
        assert!(b.contains(i), "insert ok, but not contains");
        i += 1;
    }
    assert!(b.insert(i), format!("insert({:?}) failed", i));
    assert!(b.contains(i));

    b.optimize();
    assert!(b.repr.is_map());

    assert_eq!(i as usize, Bucket::SIZE);
    assert_eq!(b.ones(), Bucket::SIZE + 1);

    while i > 0 {
        assert!(b.remove(i), format!("remove({:?}) failed", i));
        assert!(!b.contains(i));
        i -= 1;
    }
    assert!(b.remove(i), format!("remove({:?}) failed", i));
    assert_eq!(i, 0);
    assert_eq!(b.ones(), 0);

    b.optimize();
    assert!(b.repr.is_vec());

    debug!("{:?} {:?}", b, b.ones());
}

fn new_itermap<'a>(bits: &'a [u64]) -> Iter {
    let popc = bits.iter().fold(0, |acc, &x| acc + x.ones());
    Iter::map(bits, popc)
}

#[test]
fn test_iter() {
    let _ = env_logger::init();
    let vec_0 = vec![1|1<<63; 3];
    {
        let mut iter = new_itermap(&vec_0[..]);
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(63), iter.next());
        assert_eq!(Some(64), iter.next());
        assert_eq!(Some(127), iter.next());
        assert_eq!(Some(128), iter.next());
        assert_eq!(Some(191), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
    {
        let mut iter = new_itermap(&vec_0[..]);
        assert_eq!(Some(191), iter.next_back());
        assert_eq!(Some(128), iter.next_back());
        assert_eq!(Some(127), iter.next_back());
        assert_eq!(Some(64), iter.next_back());
        assert_eq!(Some(63), iter.next_back());
        assert_eq!(Some(0), iter.next_back());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
    {
        let mut iter = new_itermap(&vec_0[..]);
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(191), iter.next_back());
        assert_eq!(Some(63), iter.next());
        assert_eq!(Some(128), iter.next_back());
        assert_eq!(Some(64), iter.next());
        assert_eq!(Some(127), iter.next_back());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
    {
        let mut iter = new_itermap(&vec_0[..]);
        assert_eq!(Some(191), iter.next_back());
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(128), iter.next_back());
        assert_eq!(Some(63), iter.next());
        assert_eq!(Some(127), iter.next_back());
        assert_eq!(Some(64), iter.next());
        assert_eq!(None, iter.next_back());
        assert_eq!(None, iter.next());
    }
}

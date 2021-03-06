//! Trait for hash functions.

use std::mem;
use std::num::Wrapping;

/// Hash function.
pub trait HashFun<K> {
    /// Hash function.
    fn hash(&self, &K) -> usize;
}

/// Hash function for pairs of `usize`, using the Tomas Wang hash.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct UintPairTWHash { unused: usize }

impl UintPairTWHash {
    /// Creates a new UintPairTWHash.
    pub fn new() -> UintPairTWHash {
        UintPairTWHash { unused: 0 }
    }
}

impl HashFun<(usize, usize)> for UintPairTWHash {
    #[inline]
    fn hash(&self, &(a, b): &(usize, usize)) -> usize {
        let mut ia = a;
        let mut ib = b;

        if ia > ib {
            mem::swap(&mut ia, &mut ib)
        }

        tomas_wang_hash(key_from_pair(ia, ib))
    }
}

/// Hash function for `usize`.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct UintTWHash { unused: usize } // FIXME: ICE if the struct is zero-sized

impl UintTWHash {
    /// Creates a new UintTWHash.
    pub fn new() -> UintTWHash {
        UintTWHash { unused: 0 }
    }
}

impl<T: ::std::convert::Into<usize> + Clone> HashFun<T> for UintTWHash {
    #[inline]
    fn hash(&self, a: &T) -> usize {
        tomas_wang_hash(a.clone().into())
    }
}

/// Combines two `usize` on a single one.
#[cfg(target_pointer_width = "32")] #[inline]
pub fn key_from_pair(a: usize, b: usize) -> usize {
    (a & 0xffff) | (b << 16)
}

/// Combines two `usize` on a single one.
#[cfg(target_pointer_width = "64")] #[inline]
pub fn key_from_pair(a: usize, b: usize) -> usize {
    (a & 0xffffffff) | (b << 32)
}

// http://www.concentric.net/~Ttwang/tech/inthash.htm -- dead link!
// (this one works: http://naml.us/blog/tag/thomas-wang)
/// Tomas Wang integer hash function.
#[cfg(target_pointer_width = "64")] #[inline]
pub fn tomas_wang_hash(k: usize) -> usize {
    let mut res = Wrapping(k);

    res = res + !(res << 32);
    res = res ^ (res >> 22);
    res = res + !(res << 13);
    res = res ^ (res >> 8);
    res = res + (res << 3);
    res = res ^ (res >> 15);
    res = res + !(res << 27);
    res = res ^ (res >> 31);

    let Wrapping(res_val) = res;
    res_val
}

/// Tomas Wang integer hash function.
#[cfg(target_pointer_width = "32")] #[inline]
pub fn tomas_wang_hash(k: usize) -> usize {
    let mut res = Wrapping(k);

    res = res + !(res << 15);
    res = res ^ (res >> 10);
    res = res + (res << 3);
    res = res ^ (res >> 6);
    res = res + !(res << 11);
    res = res ^ (res >> 16);

    let Wrapping(res_val) = res;
    res_val
}

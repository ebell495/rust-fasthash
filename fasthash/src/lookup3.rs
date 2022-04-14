//! `Lookup3`, non-cryptographic hash.
//!
//! by Bob Jekins
//!
//! <http://burtleburtle.net/bob/c/lookup3.c>
//!
//!
//! # Example
//!
//! ```
//! use std::hash::{Hash, Hasher};
//!
//! use fasthash::{lookup3, Lookup3Hasher};
//!
//! fn hash<T: Hash>(t: &T) -> u64 {
//!     let mut s: Lookup3Hasher = Default::default();
//!     t.hash(&mut s);
//!     s.finish()
//! }
//!
//! let h = lookup3::hash32(b"hello world\xff");
//!
//! assert_eq!(h, hash(&"hello world") as _);
//! ```
//!
use crate::ffi;

use crate::hasher::FastHash;

/// `Lookup3` 32-bit hash functions
///
/// # Example
///
/// ```
/// use fasthash::{lookup3::Hash32, FastHash};
///
/// assert_eq!(Hash32::hash(b"hello"), 885767278);
/// assert_eq!(Hash32::hash_with_seed(b"hello", 123), 632258402);
/// assert_eq!(Hash32::hash(b"helloworld"), 1392336737);
/// ```
#[derive(Clone, Default)]
pub struct Hash32;

impl FastHash for Hash32 {
    type Hash = u32;
    type Seed = u32;

    #[inline(always)]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: T, seed: u32) -> u32 {
        unsafe {
            ffi::lookup3(
                bytes.as_ref().as_ptr() as *const _,
                bytes.as_ref().len() as _,
                seed,
            )
        }
    }
}

trivial_hasher! {
    /// # Example
    ///
    /// ```
    /// use std::hash::Hasher;
    ///
    /// use fasthash::{lookup3::Hasher32, FastHasher};
    ///
    /// let mut h = Hasher32::new();
    ///
    /// h.write(b"hello");
    /// assert_eq!(h.finish(), 885767278);
    ///
    /// h.write(b"world");
    /// assert_eq!(h.finish(), 1392336737);
    /// ```
    Hasher32(Hash32) -> u32
}

/// `Lookup3` 32-bit hash functions for a byte array.
#[inline(always)]
pub fn hash32<T: AsRef<[u8]>>(v: T) -> u32 {
    Hash32::hash(v)
}

/// `Lookup3` 32-bit hash function for a byte array.
/// For convenience, a 32-bit seed is also hashed into the result.
#[inline(always)]
pub fn hash32_with_seed<T: AsRef<[u8]>>(v: T, seed: u32) -> u32 {
    Hash32::hash_with_seed(v, seed)
}

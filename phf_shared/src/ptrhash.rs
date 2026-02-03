use siphasher::sip128::{Hash128, Hasher128, SipHasher13};

use crate::{HashKey, Hashes, PhfHash};

/// Hash values used by the PtrHash-style algorithm.
#[derive(Clone, Copy, Debug)]
pub struct PtrHashes {
    pub h1: u64,
    pub h2: u64,
}

#[inline]
pub fn hash<T: ?Sized + PhfHash>(x: &T, key: &HashKey) -> PtrHashes {
    let mut hasher = SipHasher13::new_with_keys(0, *key);
    x.phf_hash(&mut hasher);
    let Hash128 { h1, h2 } = hasher.finish128();
    PtrHashes { h1, h2 }
}

/// Convert `Hashes` (from `crate::hash` or a compatible custom hash) into `PtrHashes`.
#[inline]
pub fn hashes_to_ptr(hashes: Hashes) -> PtrHashes {
    let h1 = ((hashes.g as u64) << 32) | hashes.f1 as u64;
    let h2 = splitmix64(((hashes.f2 as u64) << 32) | hashes.g as u64);
    PtrHashes { h1, h2 }
}

#[inline]
pub fn get_index(
    hashes: &PtrHashes,
    key: &HashKey,
    buckets: u32,
    slots: u32,
    pilots: &[u16],
    remap: &[u32],
    len: usize,
) -> u32 {
    if buckets == 0 {
        return 0;
    }

    let bucket = fast_reduce(hashes.h1, buckets) as usize;
    let pilot = pilots[bucket];
    let mixed = hashes.h2 ^ mix_pilot(pilot, *key);
    let slot = reduce_pow2(mixed, slots);
    if slot < len as u32 {
        slot
    } else {
        remap[(slot - len as u32) as usize]
    }
}

#[inline]
fn fast_reduce(hash: u64, n: u32) -> u32 {
    ((hash as u128 * n as u128) >> 64) as u32
}

#[inline]
fn reduce_pow2(hash: u64, n: u32) -> u32 {
    debug_assert!(n.is_power_of_two());
    (hash as u32) & (n - 1)
}

#[inline]
fn mix_pilot(pilot: u16, seed: u64) -> u64 {
    splitmix64(seed ^ pilot as u64)
}

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

use std::iter;

use fastrand::Rng;
use phf_shared::{HashKey, PhfHash};

use phf_shared::ptrhash::PtrHashes;

const FIXED_SEED: u64 = 1234567890;
const DEFAULT_ALPHA: f64 = 0.85;
const DEFAULT_BUCKET_SIZE: usize = 3;
const MAX_PILOT_TRIES: u32 = 8192;

pub struct HashState {
    pub key: HashKey,
    pub buckets: u32,
    pub slots: u32,
    pub pilots: Vec<u16>,
    pub remap: Vec<u32>,
    pub map: Vec<usize>,
}

pub fn generate_hash<H: PhfHash>(entries: &[H]) -> HashState {
    generate_ptrhash_with_hashes(entries, phf_shared::ptrhash::hash)
}

/// Generate a ptrhash using a hash function that matches `phf_shared::ptrhash::hash`.
pub fn generate_hash_with_hash_fn<T, F>(entries: &[T], hash_fn: F) -> HashState
where
    F: Fn(&T, &HashKey) -> PtrHashes,
{
    generate_ptrhash_with_hashes(entries, hash_fn)
}

fn generate_ptrhash_with_hashes<T, F>(entries: &[T], hash_fn: F) -> HashState
where
    F: Fn(&T, &HashKey) -> PtrHashes,
{
    if entries.is_empty() {
        return HashState {
            key: 0,
            buckets: 0,
            slots: 0,
            pilots: vec![],
            remap: vec![],
            map: vec![],
        };
    }

    let mut rng = Rng::with_seed(FIXED_SEED);
    iter::repeat_with(|| rng.u64(..))
        .find_map(|seed| try_generate(entries, &hash_fn, seed))
        .expect("failed to solve PHF")
}

fn try_generate<T, F>(entries: &[T], hash_fn: &F, seed: HashKey) -> Option<HashState>
where
    F: Fn(&T, &HashKey) -> PtrHashes,
{
    let len = entries.len();
    let buckets = bucket_count(len);
    let slots = slot_count(len);
    let slots_u32 = slots as u32;
    let buckets_u32 = buckets as u32;

    let mut hashes = Vec::with_capacity(len);
    let mut buckets_vec = vec![Vec::new(); buckets];

    for (idx, entry) in entries.iter().enumerate() {
        let h = hash_fn(entry, &seed);
        let bucket = fast_reduce(h.h1, buckets_u32) as usize;
        buckets_vec[bucket].push(idx);
        hashes.push(h);
    }

    let mut bucket_order: Vec<usize> = (0..buckets).collect();
    bucket_order.sort_by_key(|&b| buckets_vec[b].len());
    bucket_order.reverse();

    let mut pilots = vec![0u16; buckets];
    let mut slots_map = vec![None; slots];
    let mut try_map = vec![0u64; slots];
    let mut generation = 0u64;
    let mut values_to_add: Vec<(usize, usize)> = Vec::new();

    'buckets: for bucket in bucket_order {
        let keys = &buckets_vec[bucket];
        if keys.is_empty() {
            continue;
        }

        for pilot in 0..=MAX_PILOT_TRIES {
            generation = generation.wrapping_add(1);
            values_to_add.clear();
            let mut failed = false;

            for &key_idx in keys {
                let slot = slot_for(&hashes[key_idx], pilot as u16, seed, slots_u32) as usize;
                if slots_map[slot].is_some() || try_map[slot] == generation {
                    failed = true;
                    break;
                }
                try_map[slot] = generation;
                values_to_add.push((slot, key_idx));
            }

            if failed || values_to_add.len() != keys.len() {
                continue;
            }

            pilots[bucket] = pilot as u16;
            for &(slot, key_idx) in &values_to_add {
                slots_map[slot] = Some(key_idx);
            }
            continue 'buckets;
        }

        return None;
    }

    let (remap, map) = build_remap_and_map(&slots_map, len);

    Some(HashState {
        key: seed,
        buckets: buckets_u32,
        slots: slots_u32,
        pilots,
        remap,
        map,
    })
}

fn build_remap_and_map(slots_map: &[Option<usize>], len: usize) -> (Vec<u32>, Vec<usize>) {
    let slots = slots_map.len();
    let mut used = vec![false; len];
    for (slot, entry) in slots_map.iter().enumerate().take(len) {
        if entry.is_some() {
            used[slot] = true;
        }
    }

    let mut free = Vec::new();
    for (idx, is_used) in used.iter().enumerate() {
        if !*is_used {
            free.push(idx as u32);
        }
    }

    let mut remap = vec![0u32; slots.saturating_sub(len)];
    let mut free_iter = free.into_iter();

    for slot in len..slots {
        if slots_map[slot].is_some() {
            if let Some(idx) = free_iter.next() {
                remap[slot - len] = idx;
            }
        }
    }

    let mut map = vec![None; len];
    for (slot, entry) in slots_map.iter().enumerate() {
        if let Some(entry_idx) = entry {
            let index = if slot < len {
                slot as u32
            } else {
                remap[slot - len]
            };
            map[index as usize] = Some(*entry_idx);
        }
    }

    let map = map
        .into_iter()
        .map(|idx| idx.expect("failed to populate ptrhash map"))
        .collect();

    (remap, map)
}

fn bucket_count(len: usize) -> usize {
    let buckets = (len + DEFAULT_BUCKET_SIZE - 1) / DEFAULT_BUCKET_SIZE;
    buckets.max(1)
}

fn slot_count(len: usize) -> usize {
    let target = ((len as f64) / DEFAULT_ALPHA).ceil() as usize;
    let target = target.max(len.max(1));
    target.next_power_of_two()
}

#[inline]
fn slot_for(hashes: &PtrHashes, pilot: u16, seed: u64, slots: u32) -> u32 {
    let mixed = hashes.h2 ^ mix_pilot(pilot, seed);
    reduce_pow2(mixed, slots)
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

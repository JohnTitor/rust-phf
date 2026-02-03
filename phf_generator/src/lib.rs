//! See [the `phf` crate's documentation][phf] for details.
//!
//! [phf]: https://docs.rs/phf

#![doc(html_root_url = "https://docs.rs/phf_generator/0.13.1")]

#[cfg(feature = "ptrhash")]
mod ptrhash;
#[cfg(feature = "ptrhash")]
pub use ptrhash::{generate_hash, generate_hash_with_hash_fn, HashState};

#[cfg(not(feature = "ptrhash"))]
mod chd;
#[cfg(not(feature = "ptrhash"))]
pub use chd::{generate_hash, generate_hash_with_hash_fn, HashState};

//! # murmur2 Hash Functions
//!
//! This is a pure Rust implementation of the murmur2 hash functions.
//! It is tested against the [original C++ implementation](https://github.com/aappleby/smhasher/).
//! To keep this crate `no_std` and dependency free,
//! the tests live in a different crate, which is not published to crates.io.
//! The implementations have not been particularly optimized for performance.
//!
//! The original C++ implementations are architecture/endianness dependent.
//! The only independent function provided is `MurmurHashNeutral2`.
//! This crate does not follow that scheme,
//! all functions are provided in an endianness-dependent and -independent version.
//! The endianness-dependent versions have their name suffixed with a `ne`, for "native endian".
#![cfg_attr(not(test), no_std)]

#[macro_use]
mod hlp;
mod imp;
#[cfg(test)]
mod test;

/// Endianness-independent `MurmurHash2`
///
/// This is the only function where an endianness-independent version is provided by the original C++ code,
/// called `MurmurHashNeutral2` there.
pub fn murmur2(data: &[u8], seed: u32) -> u32 {
    imp::murmur2(data, seed, u32::from_le_bytes)
}

/// `MurmurHash2`
pub fn murmur2ne(data: &[u8], seed: u32) -> u32 {
    imp::murmur2(data, seed, u32::from_ne_bytes)
}

/// Endianness-independent `MurmurHash2A`
pub fn murmur2a(data: &[u8], seed: u32) -> u32 {
    imp::murmur2a(data, seed, u32::from_le_bytes)
}

/// `MurmurHash2A`
pub fn murmur2ane(data: &[u8], seed: u32) -> u32 {
    imp::murmur2a(data, seed, u32::from_ne_bytes)
}

/// Endianness-independent `MurmurHash64A`
pub fn murmur64a(data: &[u8], seed: u64) -> u64 {
    imp::murmur64a(data, seed, u64::from_le_bytes)
}

/// `MurmurHash64A`
pub fn murmur64ane(data: &[u8], seed: u64) -> u64 {
    imp::murmur64a(data, seed, u64::from_ne_bytes)
}

/// Endianness-independent `MurmurHash64B`
pub fn murmur64b(data: &[u8], seed: u64) -> u64 {
    imp::murmur64b(data, seed, u32::from_le_bytes)
}

/// `MurmurHash64B`
pub fn murmur64bne(data: &[u8], seed: u64) -> u64 {
    imp::murmur64b(data, seed, u32::from_ne_bytes)
}

/// Seed found in [Kafka source](https://github.com/apache/kafka/blob/3.1.0/clients/src/main/java/org/apache/kafka/common/utils/Utils.java#L479).
// No idea where they took it from
pub const KAFKA_SEED: u32 = 0x9747b28c;

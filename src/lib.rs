#[macro_use]
mod hlp;
mod imp;
#[cfg(test)]
mod test;
use imp::*;

pub fn murmur2le(data: &[u8], seed: u32) -> u32 {
    murmur2(data, seed, u32::from_le_bytes)
}

pub fn murmur2ne(data: &[u8], seed: u32) -> u32 {
    murmur2(data, seed, u32::from_ne_bytes)
}

pub fn murmur2ale(data: &[u8], seed: u32) -> u32 {
    murmur2a(data, seed, u32::from_le_bytes)
}

pub fn murmur2ane(data: &[u8], seed: u32) -> u32 {
    murmur2a(data, seed, u32::from_ne_bytes)
}

pub fn murmur64ale(data: &[u8], seed: u64) -> u64 {
    murmur64a(data, seed, u64::from_le_bytes)
}

pub fn murmur64ane(data: &[u8], seed: u64) -> u64 {
    murmur64a(data, seed, u64::from_ne_bytes)
}
pub fn murmur64ble(data: &[u8], seed: u64) -> u64 {
    murmur64b(data, seed, u32::from_le_bytes)
}

pub fn murmur64bne(data: &[u8], seed: u64) -> u64 {
    murmur64b(data, seed, u32::from_ne_bytes)
}

/// Seed found in Kafka source.
// No idea where they took it from
pub const KAFKA_SEED: u32 = 0x9747b28c;

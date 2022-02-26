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

/// Seed found in Kafka source.
// No idea where they took it from
pub const KAFKA_SEED: u32 = 0x9747b28c;

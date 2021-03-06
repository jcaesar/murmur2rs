use crate::hlp::*;

// Mixing consts
const M_64: u64 = 0xc6a4a7935bd1e995;
const M_32: u32 = M_64 as u32;

pub fn murmur2(data: &[u8], seed: u32, load: impl Fn([u8; 4]) -> u32) -> u32 {
    let h = seed ^ (data.len() as u32);

    let mut chunks = data.chunks_exact(4);
    let h = (&mut chunks).fold(h, |h, k| round!(M_32, h, load(k.try_into().unwrap())));
    let h = short_round!(M_32, h, chunks.remainder(), u32);

    h.slack(13).wrapping_mul(M_32).slack(15)
}

pub fn murmur64a(data: &[u8], seed: u64, load: impl Fn([u8; 8]) -> u64) -> u64 {
    let h = seed ^ (data.len() as u64).wrapping_mul(M_64); // Subtle Difference 1

    let mut chunks = data.chunks_exact(8);
    let h = (&mut chunks).fold(h, |h, k| {
        // Subtle difference 2
        (h ^ load(k.try_into().unwrap())
            .wrapping_mul(M_64)
            .slack(47)
            .wrapping_mul(M_64))
        .wrapping_mul(M_64)
    });
    let h = short_round!(M_64, h, chunks.remainder(), u64);

    h.slack(47).wrapping_mul(M_64).slack(47)
}

pub fn murmur64b(data: &[u8], seed: u64, load: impl Fn([u8; 4]) -> u32) -> u64 {
    let h1 = (seed as u32) ^ (data.len() as u32);
    let h2 = (seed >> 32) as u32;

    let mut chunks = data.chunks_exact(4);
    let (h1, h2) = (&mut chunks).fold((h1, h2), |(h1, h2), k| {
        (h2, round!(M_32, h1, load(k.try_into().unwrap())))
    });
    let (h1, h2) = match data.len() % 8 > 3 {
        true => (h2, h1),
        false => (h1, h2),
    };
    let h2 = short_round!(M_32, h2, chunks.remainder(), u32);

    let h1 = (h1 ^ h2 >> 18).wrapping_mul(M_32);
    let h2 = (h2 ^ h1 >> 22).wrapping_mul(M_32);
    let h1 = (h1 ^ h2 >> 17).wrapping_mul(M_32);
    let h2 = (h2 ^ h1 >> 19).wrapping_mul(M_32);

    ((h1 as u64) << 32) | h2 as u64
}

pub fn murmur2a(data: &[u8], seed: u32, load: impl Fn([u8; 4]) -> u32) -> u32 {
    let mut chunks = data.chunks_exact(4);
    let h = (&mut chunks).fold(seed, |h, k| round!(M_32, h, load(k.try_into().unwrap())));
    let t = rest!(chunks.remainder(), u32);

    round!(M_32, round!(M_32, h, t), data.len() as u32)
        .slack(13)
        .wrapping_mul(M_32)
        .slack(15)
}

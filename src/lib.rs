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

// Integers in rust aren't generic. :/
macro_rules! round {
    ($m:expr, $h:expr, $k:expr) => {
        $k.wrapping_mul($m).slack(24).wrapping_mul($m) ^ $h.wrapping_mul($m)
    };
}

macro_rules! rest {
    ($r:expr, $T:ty) => {
        $r.iter().rev().fold(0, |r, &i| (i as $T) | (r << 8))
    };
}

macro_rules! short_round {
    ($m:expr, $h:expr, $r:expr, $T:ty) => {{
        let r = $r;
        match r.is_empty() {
            false => ($h ^ rest!(r, $T)).wrapping_mul($m),
            true => $h,
        }
    }};
}

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

trait Slack {
    fn slack(self, slack: Self) -> Self
    where
        Self: Sized;
}
macro_rules! slack {
    ($typ:ty) => {
        impl Slack for $typ {
            fn slack(self, slack: Self) -> Self {
                self ^ self >> slack
            }
        }
    };
}
slack!(u32);
slack!(u64);

#[cfg(test)]
mod tests {
    fn murmur2(data: &[u8]) -> u32 {
        super::murmur2le(data, super::KAFKA_SEED)
    }

    #[test]
    fn kafka_test_vecs() {
        assert_eq!(murmur2("".as_bytes()), 275646681);
        assert_eq!(murmur2("m".as_bytes()), 1063097864);
        assert_eq!(murmur2("mu".as_bytes()), 3903436272);
        assert_eq!(murmur2("mur".as_bytes()), 2210029508);
        assert_eq!(murmur2("murm".as_bytes()), 519236232);
        assert_eq!(murmur2("murmu".as_bytes()), 3401258662);
        assert_eq!(murmur2("murmur".as_bytes()), 322215093);
        assert_eq!(murmur2("murmur2".as_bytes()), 1394823152);
        assert_eq!(murmur2("murmur2".as_bytes()), 1394823152);
        assert_eq!(murmur2(&[1, 2, 3, 4, 0, 252, 253, 254, 255]), 3855791143);
    }
}

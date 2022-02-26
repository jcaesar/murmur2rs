/// Seed found in Kafka source.
// No idea where they took it from
pub const KAFKA_SEED: u32 = 0x9747b28c;

pub fn murmur2(data: &[u8], seed: u32) -> u32 {
    let len = data.len();
    let m = 0x5bd1e995u32;
    let h = seed ^ (len as u32);

    let mut chunks = data.chunks_exact(4);
    let h = (&mut chunks).fold(h, |h, k| {
        u32::from_le_bytes(k.try_into().unwrap())
            .wrapping_mul(m)
            .slack(24)
            .wrapping_mul(m)
            ^ h.wrapping_mul(m)
    });

    let r = chunks.remainder();
    let h = match r.is_empty() {
        false => (h ^ r.iter().rev().fold(0, |r, &i| (i as u32) | (r << 8))).wrapping_mul(m),
        true => h,
    };

    h.slack(13).wrapping_mul(m).slack(15)
}

pub fn murmur64a(data: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let m = 0xc6a4a7935bd1e995u64;
    let h = seed ^ (len as u64).wrapping_mul(m); // Subtle Difference 1

    let mut chunks = data.chunks_exact(8);
    let h = (&mut chunks).fold(h, |h, k| {
        // Subtle difference 2
        (h ^ u64::from_le_bytes(k.try_into().unwrap())
            .wrapping_mul(m)
            .slack(47)
            .wrapping_mul(m))
        .wrapping_mul(m)
    });

    let r = chunks.remainder();
    let h = match r.is_empty() {
        false => (h ^ r.iter().rev().fold(0, |r, &i| (i as u64) | (r << 8))).wrapping_mul(m),
        true => h,
    };

    h.slack(47).wrapping_mul(m).slack(47)
}

pub fn murmur64b(data: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let m = 0x5bd1e995u32;
    let h1 = (seed as u32) ^ (len as u32);
    let h2 = (seed >> 32) as u32;

    let mut chunks = data.chunks_exact(4);
    let (h1, h2) = (&mut chunks).fold((h1, h2), |(h1, h2), k| {
        (
            h2,
            u32::from_le_bytes(k.try_into().unwrap())
                .wrapping_mul(m)
                .slack(24)
                .wrapping_mul(m)
                ^ h1.wrapping_mul(m),
        )
    });

    let (h1, h2) = match len % 8 > 3 {
        true => (h2, h1),
        false => (h1, h2),
    };

    let r = chunks.remainder();
    let h2 = match r.is_empty() {
        false => (h2 ^ r.iter().rev().fold(0, |r, &i| (i as u32) | (r << 8))).wrapping_mul(m),
        true => h2,
    };

    let h1 = (h1 ^ h2 >> 18).wrapping_mul(m);
    let h2 = (h2 ^ h1 >> 22).wrapping_mul(m);
    let h1 = (h1 ^ h2 >> 17).wrapping_mul(m);
    let h2 = (h2 ^ h1 >> 19).wrapping_mul(m);

    ((h1 as u64) << 32) | h2 as u64
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
        super::murmur2(data, super::KAFKA_SEED)
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

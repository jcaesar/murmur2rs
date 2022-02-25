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

trait Slack {
    fn slack(self, slack: Self) -> Self
    where
        Self: Sized;
}

impl Slack for u32 {
    fn slack(self, slack: Self) -> Self {
        self ^ self >> slack
    }
}

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

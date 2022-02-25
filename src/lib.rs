pub fn murmur2(data: &[u8]) -> u32 {
    let len = data.len();
    let seed = 0x9747b28cu32;
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

    let rem = match chunks.remainder() {
        [] => [0, 0, 0, 0],
        [a] => [*a, 0, 0, 0],
        [a, b] => [*a, *b, 0, 0],
        [a, b, c] => [*a, *b, *c, 0],
        _ => unreachable!(),
    };
    let h = h ^ u32::from_le_bytes(rem);
    let h = match rem.is_empty() {
        false => h.wrapping_mul(m),
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
    use super::*;
    #[test]
    fn java_test_vecs() {
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

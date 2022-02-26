fn main() {
    panic!("cargo test only!")
}

mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(deref_nullptr)] // WTF, actually…
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use super::c;
    use murmur2::KAFKA_SEED;
    use std::ffi::c_void;
    use std::fmt::Debug;

    const SEED_64: u64 = 0x0123456789abcdef;

    fn test<T: Eq + Debug>(fut: impl Fn(&[u8]) -> T, against: impl Fn(*const c_void, i32) -> T) {
        let dat = "murmur2murmur2murmur2".as_bytes();
        for s in 0..dat.len() {
            let dat = &dat[s..];
            for e in 0..dat.len() {
                let dat = &dat[..e];
                assert_eq!(
                    fut(dat),
                    against(dat.as_ptr() as *const c_void, dat.len() as i32),
                    "Input: {:?}",
                    std::str::from_utf8(dat).unwrap()
                );
            }
        }
    }

    #[test]
    fn murmur2() {
        test(
            |dat| murmur2::murmur2le(dat, KAFKA_SEED),
            |p, len| unsafe { c::cMurmurHashNeutral2(p, len, KAFKA_SEED) },
        );
    }
    #[test]
    fn murmur2ne() {
        test(
            |dat| murmur2::murmur2ne(dat, KAFKA_SEED),
            |p, len| unsafe { c::cMurmurHash2(p, len, KAFKA_SEED) },
        );
    }
    #[cfg(target_endian = "little")]
    mod le {
        use super::*;

        // On little endian architectures, MurmurHash2 and MurmurHashNeutral2 should be equal…
        #[test]
        fn equal() {
            test(
                |dat| murmur2::murmur2le(dat, KAFKA_SEED),
                |p, len| unsafe { c::cMurmurHash2(p, len, KAFKA_SEED) },
            );
        }
        // Lastly, there is also MurmurHashAligned2,
        // which is (wongly?) claimed to be equal to MurmurHash2.
        // (If I read the code correctly, that again only holds on little endian systems.)
        #[test]
        fn aligned() {
            test(
                |dat| murmur2::murmur2le(dat, KAFKA_SEED),
                |p, len| unsafe { c::cMurmurHashAligned2(p, len, KAFKA_SEED) },
            );
        }
    }

    #[test]
    fn murmur64a() {
        test(
            |dat| murmur2::murmur64ane(dat, SEED_64),
            |p, len| unsafe { c::cMurmurHash64A(p, len, SEED_64) },
        );
    }

    #[test]
    fn murmur64b() {
        test(
            |dat| murmur2::murmur64bne(dat, SEED_64),
            |p, len| unsafe { c::cMurmurHash64B(p, len, SEED_64) },
        );
    }

    #[test]
    fn murmur2a() {
        test(
            |dat| murmur2::murmur2ane(dat, KAFKA_SEED),
            |p, len| unsafe { c::cMurmurHash2A(p, len, KAFKA_SEED) },
        );
    }
}

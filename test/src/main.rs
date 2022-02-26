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
            |dat| murmur2::murmur2(dat, KAFKA_SEED),
            |p, len| unsafe { super::c::cMurmurHash2(p, len, KAFKA_SEED) },
        );
    }

    #[test]
    fn murmur64a() {
        test(
            |dat| murmur2::murmur64a(dat, SEED_64),
            |p, len| unsafe { super::c::cMurmurHash64A(p, len, SEED_64) },
        );
    }

    #[test]
    fn murmur64b() {
        test(
            |dat| murmur2::murmur64b(dat, SEED_64),
            |p, len| unsafe { super::c::cMurmurHash64B(p, len, SEED_64) },
        );
    }
}

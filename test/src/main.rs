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
    use std::ffi::c_void;

    fn sampledata() -> impl Iterator<Item = &'static [u8]> {
        let dat = "murmur2murmur2murmur2".as_bytes();
        (0..dat.len()).flat_map(move |s| (s..dat.len()).map(move |e| &dat[s..e]))
    }

    #[test]
    fn murmur2() {
        for dat in sampledata() {
            assert_eq!(
                murmur2::murmur2(dat, murmur2::KAFKA_SEED),
                unsafe {
                    super::c::cMurmurHash2(
                        dat.as_ptr() as *const c_void,
                        dat.len() as i32,
                        murmur2::KAFKA_SEED,
                    )
                },
                "Input: {:?}",
                std::str::from_utf8(dat).unwrap()
            );
        }
    }

    #[test]
    fn murmur64a() {
        for dat in sampledata() {
            let seed = 0x0123456789abcdef;
            assert_eq!(
                murmur2::murmur64a(dat, seed),
                unsafe {
                    super::c::cMurmurHash64A(dat.as_ptr() as *const c_void, dat.len() as i32, seed)
                },
                "Input: {:?}",
                std::str::from_utf8(dat).unwrap()
            );
        }
    }

    #[test]
    fn murmur64b() {
        for dat in sampledata() {
            let seed = 0x0123456789abcde;
            assert_eq!(
                murmur2::murmur64b(dat, seed),
                unsafe {
                    super::c::cMurmurHash64B(dat.as_ptr() as *const c_void, dat.len() as i32, seed)
                },
                "Input: {:?}",
                std::str::from_utf8(dat).unwrap()
            );
        }
    }
}

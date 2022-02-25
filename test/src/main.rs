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

    #[test]
    fn murmur2() {
        let dat = "murmur2murmur2murmur2".as_bytes();
        for s in 0..dat.len() {
            let dat = &dat[s..];
            for e in 0..dat.len() {
                let dat = &dat[..e];
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
    }
}

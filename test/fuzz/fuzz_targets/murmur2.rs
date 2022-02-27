#![no_main]
use core::ffi::c_void;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 4 {
        let seed = u32::from_ne_bytes(data[..4].try_into().unwrap());
        let data = &data[4..];
        let ptr = data.as_ptr() as *const c_void;
        let len = data.len() as i32;
        assert_eq!(murmur2::murmur2(data, seed), unsafe {
            murmur2_test::c::cMurmurHashNeutral2(ptr, len, seed)
        });
        assert_eq!(murmur2::murmur2ne(data, seed), unsafe {
            murmur2_test::c::cMurmurHash2(ptr, len, seed)
        });
        assert_eq!(murmur2::murmur2ane(data, seed), unsafe {
            murmur2_test::c::cMurmurHash2A(ptr, len, seed)
        });

        if cfg!(target_endianness = "little") {
            assert_eq!(murmur2::murmur2a(data, seed), unsafe {
                murmur2_test::c::cMurmurHash2A(ptr, len, seed)
            });
        }
    }

    if data.len() >= 8 {
        let seed = u64::from_ne_bytes(data[..8].try_into().unwrap());
        let data = &data[8..];
        let ptr = data.as_ptr() as *const c_void;
        let len = data.len() as i32;
        assert_eq!(murmur2::murmur64ane(data, seed), unsafe {
            murmur2_test::c::cMurmurHash64A(ptr, len, seed)
        });
        assert_eq!(murmur2::murmur64bne(data, seed), unsafe {
            murmur2_test::c::cMurmurHash64B(ptr, len, seed)
        });

        if cfg!(target_endianness = "little") {
            assert_eq!(murmur2::murmur64a(data, seed), unsafe {
                murmur2_test::c::cMurmurHash64A(ptr, len, seed)
            });
            assert_eq!(murmur2::murmur64b(data, seed), unsafe {
                murmur2_test::c::cMurmurHash64B(ptr, len, seed)
            });
        }
    }
});

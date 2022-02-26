macro_rules! test {
    ([$fle:ident, $fne:ident], ($emp:expr, $mur:expr, $range:expr)) => {
        #[test]
        fn $fle() {
            let seed = 0x12345678_9747b28cu64 as _;
            #[cfg(not(target_endian = "little"))]
            let fs = [crate::$fle];
            #[cfg(target_endian = "little")]
            let fs = [crate::$fle, crate::$fne];
            for f in fs {
                assert_eq!(f(&[], seed), $emp);
                assert_eq!(f("murmur2".as_bytes(), seed), $mur);
                assert_eq!(f(&(1..255).collect::<Vec<_>>(), seed), $range);
            }
        }
    };
}

test!([murmur2le, murmur2ne], (275646681, 1394823152, 2950539382));
test!(
    [murmur2ale, murmur2ane],
    (3816574809, 2735559800, 2216743126)
);
test!(
    [murmur64ale, murmur64ane],
    (4971528555162033992, 443646041180781888, 528696902255405805)
);
test!(
    [murmur64ble, murmur64bne],
    (
        13704036419649912789,
        16557411989772816646,
        16142286494200638697
    )
);

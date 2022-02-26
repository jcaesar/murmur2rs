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

pub trait Slack {
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

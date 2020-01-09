use rand::random;
use std::cmp::{Eq, Ordering, PartialEq};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Id {
    high: u128,
    low: u128,
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}{:032x}", self.high, self.low)
    }
}

impl Id {
    pub fn new(h: u128, l: u128) -> Self {
        Self { high: h, low: l }
    }

    pub fn zero() -> Self {
        Self { high: 0, low: 0 }
    }

    pub fn max() -> Self {
        Self {
            high: u128::max_value(),
            low: u128::max_value(),
        }
    }

    pub fn rand() -> Self {
        Self {
            high: random::<u128>(),
            low: random::<u128>(),
        }
    }

    pub fn half(self) -> Id {
        let mut half = self;
        let rls_overflow: u128 = ((half.high & 0b1) as u128) << 127;
        // Shift right to divide by 2
        half.high = half.high >> 1;
        half.low = half.low >> 1;

        // Add overflow bit from high to low
        half.low |= rls_overflow;

        half
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Id) -> Ordering {
        match self.high {
            0 => self.low.cmp(&other.low),
            _ => self.high.cmp(&other.high),
        }
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_add_id {
    ($($t:ty)*) => ($(
        impl Add<$t> for Id {
            type Output = Self;
            fn add(self, rhs: $t) -> Self {
                let mut rhs = rhs;

                let mut high = self.high;
                let mut low = self.low;

                // If low is going to overflow
                if self.low.checked_add(rhs as u128) == None {
                    rhs -= (u128::max_value() - self.low) as $t;
                    low = 0;
                    // If high is going to overflow too
                    if rhs as u128 > (u128::max_value() - self.high) as u128 {
                        high = u128::max_value();
                        low = u128::max_value();
                    }
                    // If high does not overflow just add it
                    else {
                        high += rhs as u128;
                    }
                }
                // If low does not overflow just add it
                else {
                    low += rhs as u128;
                }
                Self {high: high, low: low}
            }
        }
    )*)
}
impl_add_id! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_adda_id {
    ($($t:ty)*) => ($(
        impl AddAssign<$t> for Id {
            fn add_assign(&mut self, rhs: $t) {
                *self = *self + rhs
            }
        }
    )*)
}
impl_adda_id! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_sub_id {
    ($($t:ty)*) => ($(
        impl Sub<$t> for Id {
            type Output = Self;
            fn sub(self, rhs: $t) -> Self {
                let mut rhs = rhs as u128;

                let mut high = self.high;
                let mut low = self.low;

                // If low is going to underflow
                if rhs > self.low {
                    rhs -= self.low + 1;
                    match high {
                        0 => { high = 0; low = 0; },
                        n => { high = n - 1; low = u128::max_value() - rhs },
                    };
                }
                // If low does not underflow just sub it
                else {
                    low -= rhs;
                }

                Self {high: high, low: low}
            }
        }
    )*)
}
impl_sub_id! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! impl_suba_id {
    ($($t:ty)*) => ($(
        impl SubAssign<$t> for Id {
            fn sub_assign(&mut self, rhs: $t) {
                *self = *self - rhs
            }
        }
    )*)
}
impl_suba_id! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

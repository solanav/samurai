// Kademlia ID
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::cmp::{Eq, PartialEq, Ordering};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Id {
    high: u32,
    low: u128
}

impl Id {
    pub fn new(h: u32, l: u128) -> Self {
        Self {
            high: h,
            low: l
        }
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
                    if rhs as u128 > (u32::max_value() - self.high) as u128 {
                        high = u32::max_value();
                        low = u128::max_value();
                    }
                    // If high does not overflow just add it
                    else {
                        high += rhs as u32;
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
                    rhs -= self.low;
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
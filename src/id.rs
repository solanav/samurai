// Kademlia ID
use std::ops::Add;
use std::cmp::Eq;
use std::cmp::PartialEq;

#[derive(Debug, Eq, PartialEq, Clone)]
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
                        rhs -= (u32::max_value() - self.high) as $t;
                        high = 0;
                        low += rhs as u128;
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
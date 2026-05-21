use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct OffsetUtf16(pub usize);

impl AddAssign<Self> for OffsetUtf16 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

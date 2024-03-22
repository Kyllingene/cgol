use std::ops::{Add, AddAssign};

use bytemuck::{Pod, Zeroable};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C, align(4))]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub _pad: u8,
}

impl Pixel {
    pub fn is_live(&self) -> bool {
        self.r != 0
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Self) -> Self::Output {
        crate::pix(
            self.r.wrapping_add(rhs.r),
            self.g.wrapping_add(rhs.g),
            self.b.wrapping_add(rhs.b),
        )
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

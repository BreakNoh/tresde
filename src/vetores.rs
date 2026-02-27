use derive_more::{Add, Div, Into, Mul, Rem, Sub};
use num_traits::Num;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Add, Sub, Mul, Div, Rem, Into)]
pub struct Vec2<T: Num> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Add, Sub, Mul, Div, Rem, Into)]
pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T: Num> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

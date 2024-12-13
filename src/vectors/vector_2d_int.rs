use num_traits::int::PrimInt;
use serde::Deserialize;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

// TODO: Put Deserialize behind a feature flag.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vector2DInt<T: PrimInt> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq + PrimInt> Eq for Vector2DInt<T> {}

impl<T: Hash + PrimInt> Hash for Vector2DInt<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Vector2DInt<u32> {
    pub fn to_i32(self) -> Vector2DInt<i32> {
        Vector2DInt {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl<T: PrimInt> Add for Vector2DInt<T>
where
    T: Into<i32>,
{
    type Output = Vector2DInt<i32>;

    fn add(self, other: Vector2DInt<T>) -> Vector2DInt<i32> {
        Vector2DInt {
            x: self.x.into() + other.x.into(),
            y: self.y.into() + other.y.into(),
        }
    }
}

impl<T: PrimInt> Sub for Vector2DInt<T>
where
    T: Into<i32>,
{
    type Output = Vector2DInt<i32>;

    fn sub(self, other: Vector2DInt<T>) -> Vector2DInt<i32> {
        Vector2DInt {
            x: self.x.into() - other.x.into(),
            y: self.y.into() - other.y.into(),
        }
    }
}

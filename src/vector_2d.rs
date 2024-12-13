use num_traits::Float;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq> Eq for Vector2D<T> {}

impl<T: Hash> Hash for Vector2D<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

// TODO: See about generalizing this.
impl Vector2D<u32> {
    pub fn to_i32(self) -> Vector2D<i32> {
        Vector2D {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl<T> Vector2D<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd + Float,
{
    pub fn magnitude(&self) -> T {
        let sum_of_squares = self.x * self.x + self.y * self.y;
        sum_of_squares.sqrt()
    }

    pub fn normalize(&self) -> Vector2D<T> {
        let magnitude = self.magnitude();

        if magnitude == T::from(0.0).unwrap() {
            return Vector2D {
                x: T::from(1.0).unwrap(),
                y: T::from(0.0).unwrap(),
            };
        }

        Vector2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}

impl<T> Add for Vector2D<T>
where
    T: Into<i32>,
{
    type Output = Vector2D<i32>;

    fn add(self, other: Vector2D<T>) -> Vector2D<i32> {
        Vector2D {
            x: self.x.into() + other.x.into(),
            y: self.y.into() + other.y.into(),
        }
    }
}

impl<T> Sub for Vector2D<T>
where
    T: Into<i32>,
{
    type Output = Vector2D<i32>;

    fn sub(self, other: Vector2D<T>) -> Vector2D<i32> {
        Vector2D {
            x: self.x.into() - other.x.into(),
            y: self.y.into() - other.y.into(),
        }
    }
}

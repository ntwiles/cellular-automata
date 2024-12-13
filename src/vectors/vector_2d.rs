use num_traits::Float;
use serde::Deserialize;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub};

// TODO: Put Deserialize behind a feature flag.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vector2D<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq + Float> Eq for Vector2D<T> {}

impl<T: Hash + Float> Hash for Vector2D<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
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

impl<T: Float> Add for Vector2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Float> Sub for Vector2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

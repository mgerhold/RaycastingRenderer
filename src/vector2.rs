use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Copy, Clone, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_angle(radians: f64) -> Self {
        Self {
            x: radians.cos(),
            y: radians.sin(),
        }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalized(self) -> Option<Self> {
        let magnitude = self.magnitude();
        match magnitude == 0.0 {
            true => None,
            false => Some(self * (1.0 / magnitude)),
        }
    }

    pub fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn rotated(self, radians: f64) -> Self {
        let sin = radians.sin();
        let cos = radians.cos();
        Self::new(cos * self.x - sin * self.y, sin * self.x + cos * self.y)
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vector2::new(x, y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

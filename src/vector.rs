use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Represents a 2-dimensional vector of floats. Typically used to represent a point
/// on a track.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2D(pub f64, pub f64);

impl Vector2D {
    /// Returns the length^2 of this vector. If you need the
    /// actual length, use [`f64::sqrt`].
    pub fn length_squared(self) -> f64 {
        self.dot_product(self)
    }

    /// Returns the distance squared between two points. If you need the
    /// actual distance, use [`f64::sqrt`].
    pub fn distance_squared(self, other: Vector2D) -> f64 {
        let diff = other - self;

        diff.dot_product(diff)
    }

    /// Returns the dot product of the two vectors.
    pub fn dot_product(self, rhs: Vector2D) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    /// Returns the length of this vector projected onto `other`.
    pub fn length_projected_onto(self, other: Vector2D) -> f64 {
        self.dot_product(other.normalize())
    }

    /// Cross products don't exist in 2d space, however this method
    /// returns the length of what the cross product would be in
    /// 3d space. Not sure if this is even useful.
    pub fn cross_product_length(self, rhs: Vector2D) -> f64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    /// Rotates the vector 90 degrees to the left (counter-clockwise)
    pub fn rotate90_left(self) -> Vector2D {
        Vector2D(-self.1, self.0)
    }

    /// Rotates the vector 90 degrees to the right (clockwise)
    pub fn rotate90_right(self) -> Vector2D {
        Vector2D(self.1, -self.0)
    }

    /// Rotates the vector by some arbitrary number of radians
    pub fn rotate_rad(self, radians: f64) -> Vector2D {
        let sin_angle = f64::sin(radians);
        let cos_angle = f64::cos(radians);

        Vector2D(
            self.0 * cos_angle - self.1 * sin_angle,
            self.0 * sin_angle - self.1 * cos_angle,
        )
    }

    /// Returns the number of radians from (1, 0).
    pub fn angle(self) -> f64 {
        f64::atan2(self.1, self.0)
    }

    /// Returns the number of radians between the two vectors.
    pub fn angle_between(self, other: Vector2D) -> f64 {
        f64::atan2(self.cross_product_length(other), self.dot_product(other))
    }

    /// Returns the unit vector pointing in the same direction.
    pub fn normalize(self) -> Vector2D {
        let distance = self.length_squared().sqrt();

        Vector2D(self.0 / distance, self.1 / distance)
    }
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<(f64, f64)> for Vector2D {
    fn from(tuple: (f64, f64)) -> Self {
        Vector2D(tuple.0, tuple.1)
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self {
        Vector2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        Vector2D(-self.0, -self.1)
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 / rhs, self.1 / rhs)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl DivAssign<f64> for Vector2D {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl Hash for Vector2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
        self.1.to_bits().hash(state);
    }
}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits() && self.1.to_bits() == other.1.to_bits()
    }
}
impl Eq for Vector2D {}

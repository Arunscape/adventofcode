#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point(pub isize, pub isize);

impl std::ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Mul for Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Self;
    fn mul(self, other: isize) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl std::ops::Div for Point {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

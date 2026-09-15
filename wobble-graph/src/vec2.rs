
#[derive(
    Clone,
    Copy,
    Default,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Mul,
    derive_more::MulAssign,
    derive_more::Div,
    derive_more::DivAssign,
)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn distance_to(self, other: Self) -> f64 {
        (self - other).len()
    }

    pub fn direction_to(self, other: Self) -> Self {
        (other - self).normalized()
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.len();

        if len <= 0.001 {
            Self { x: 1.0, y: 0.0 }
        } else {
            self / self.len()
        }
    }
}
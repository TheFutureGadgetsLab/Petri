use std::ops::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn dot(&self, o: Vec2) -> f32 {
        (self.x * o.x) + (self.y * o.y)
    }

    #[inline]
    pub fn len_sq(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        self.x = self.x.clamp(min.x, max.x);
        self.y = self.y.clamp(min.y, max.y);
    }

    #[inline]
    pub fn zeros() -> Self {
        Self::new(0.0, 0.0)
    }

    #[inline]
    pub fn ones() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(vals: [f32; 2]) -> Self {
        Self::new(vals[0], vals[1])
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, o: Vec2) -> Self {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, o: f32) -> Vec2 {
        Vec2::new(self.x + o, self.y + o)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, o: Vec2) {
        self.x += o.x;
        self.y += o.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, o: Vec2) -> Self {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, o: f32) -> Vec2 {
        Vec2::new(self.x - o, self.y - o)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, o: Vec2) {
        self.x -= o.x;
        self.y -= o.y;
    }
}

impl Mul for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, o: Vec2) -> Self {
        Vec2::new(self.x * o.x, self.y * o.y)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, o: Vec2) -> Vec2 {
        Vec2::new(self * o.x, self * o.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, o: f32) -> Vec2 {
        Vec2::new(self.x * o, self.y * o)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    #[inline]
    fn div(self, o: f32) -> Vec2 {
        Vec2::new(self.x / o, self.y / o)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    #[inline]
    fn neg(self) -> Vec2 {
        self * -1.0
    }
}

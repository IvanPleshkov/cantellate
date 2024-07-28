use num_traits::cast::FromPrimitive;
use num_traits::cast::ToPrimitive;
use num_traits::float::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> From<[f32; 3]> for Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    fn from(array: [f32; 3]) -> Self {
        Self {
            x: N::from_f32(array[0]).unwrap_or_default(),
            y: N::from_f32(array[1]).unwrap_or_default(),
            z: N::from_f32(array[2]).unwrap_or_default(),
        }
    }
}

impl<N> From<Vec3<N>> for [f32; 3]
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    fn from(v: Vec3<N>) -> Self {
        [
            N::to_f32(&v.x).unwrap_or_default(),
            N::to_f32(&v.y).unwrap_or_default(),
            N::to_f32(&v.z).unwrap_or_default(),
        ]
    }
}

impl<N> std::ops::Add for Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<N> std::ops::Sub for Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<N> std::ops::Div<N> for Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    type Output = Self;

    fn div(self, other: N) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<N> std::ops::Mul<N> for Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<N> Vec3<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            x: N::zero(),
            y: N::zero(),
            z: N::zero(),
        }
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Vec3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }

    #[inline]
    pub fn length(self) -> N {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn normalize(self, epsilon: N) -> Option<Self> {
        let length = self.length();
        if length > epsilon {
            Some(Vec3::new(self.x / length, self.y / length, self.z / length))
        } else {
            None
        }
    }
}

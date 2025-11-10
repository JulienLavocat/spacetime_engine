use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use landmass::Vec3 as LmVec3;
use serde::{Deserialize, Serialize};
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug, Copy, PartialEq, Default, Deserialize, Serialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn splat(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
        }
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign<f32> for &mut Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl From<&Vec3> for Vec3 {
    fn from(v: &Vec3) -> Self {
        *v
    }
}

impl From<LmVec3> for Vec3 {
    fn from(vec: LmVec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<Vec3> for LmVec3 {
    fn from(st_vec: Vec3) -> Self {
        Self {
            x: st_vec.x,
            y: st_vec.y,
            z: st_vec.z,
        }
    }
}

impl From<&LmVec3> for Vec3 {
    fn from(vec: &LmVec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<&Vec3> for LmVec3 {
    fn from(st_vec: &Vec3) -> Self {
        Self {
            x: st_vec.x,
            y: st_vec.y,
            z: st_vec.z,
        }
    }
}

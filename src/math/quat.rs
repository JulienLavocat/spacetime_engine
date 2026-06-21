use parry3d::math::Rot3;
use serde::{Deserialize, Serialize};
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Debug, Copy, PartialEq, Default, Deserialize, Serialize)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
}

impl From<Rot3> for Quat {
    fn from(rot: Rot3) -> Self {
        Quat {
            x: rot.x,
            y: rot.y,
            z: rot.z,
            w: rot.w,
        }
    }
}

impl From<&Rot3> for Quat {
    fn from(rot: &Rot3) -> Self {
        Quat {
            x: rot.x,
            y: rot.y,
            z: rot.z,
            w: rot.w,
        }
    }
}

impl From<Quat> for Rot3 {
    fn from(quat: Quat) -> Self {
        Rot3::from_xyzw(quat.x, quat.y, quat.z, quat.w)
    }
}

impl From<&Quat> for Rot3 {
    fn from(quat: &Quat) -> Self {
        Rot3::from_xyzw(quat.x, quat.y, quat.z, quat.w)
    }
}

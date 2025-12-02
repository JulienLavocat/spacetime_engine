use parry3d::na::UnitQuaternion;
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

impl From<UnitQuaternion<f32>> for Quat {
    fn from(unit_quat: UnitQuaternion<f32>) -> Self {
        let quat = unit_quat.quaternion();
        Quat {
            w: quat.w,
            x: quat.i,
            y: quat.j,
            z: quat.k,
        }
    }
}

impl From<&UnitQuaternion<f32>> for Quat {
    fn from(unit_quat: &UnitQuaternion<f32>) -> Self {
        let quat = unit_quat.quaternion();
        Quat {
            w: quat.w,
            x: quat.i,
            y: quat.j,
            z: quat.k,
        }
    }
}

impl From<Quat> for UnitQuaternion<f32> {
    fn from(quat: Quat) -> Self {
        UnitQuaternion::from_quaternion(parry3d::na::Quaternion::new(
            quat.w, quat.x, quat.y, quat.z,
        ))
    }
}

impl From<&Quat> for UnitQuaternion<f32> {
    fn from(quat: &Quat) -> Self {
        UnitQuaternion::from_quaternion(parry3d::na::Quaternion::new(
            quat.w, quat.x, quat.y, quat.z,
        ))
    }
}

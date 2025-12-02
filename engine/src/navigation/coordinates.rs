use crate::math::Vec3;
use landmass::{CoordinateSystem, PointSampleDistance3d};
use serde::{Deserialize, Serialize};

/// A coordinate system matching convention: X right, Y up, Z forward.
#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize)]
pub struct XYZ;

impl CoordinateSystem for XYZ {
    type Coordinate = Vec3;
    type SampleDistance = PointSampleDistance3d;

    /// Swapping Y and Z flips handedness, so tell Landmass to flip polygons
    /// to keep them counter-clockwise.
    const FLIP_POLYGONS: bool = false;

    /// Converts from this coordinate system to Landmass’s internal Z-up one.
    /// (x, y, z) → Landmass (x, z, y)
    fn to_landmass(v: &Self::Coordinate) -> landmass::Vec3 {
        landmass::Vec3::new(v.x, v.z, v.y)
    }

    /// Converts from Landmass’s internal coordinates back to this system.
    /// Landmass (x, y, z) → (x, z, y)
    fn from_landmass(v: &landmass::Vec3) -> Self::Coordinate {
        Vec3::new(v.x, v.z, v.y)
    }
}

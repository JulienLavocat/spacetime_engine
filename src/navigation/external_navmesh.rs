use bon::Builder;
use landmass::NavigationMesh;
use serde::Deserialize;
use spacetimedb::SpacetimeType;

use crate::{math::Vec3, navigation::coordinates::XYZ};

pub type NavMeshId = u64;

#[derive(Clone, Debug, Builder, Deserialize, SpacetimeType)]
pub struct ExternalNavMesh {
    pub translation: Vec3,
    pub rotation: f32,
    pub vertices: Vec<Vec3>,
    pub polygons: Vec<Vec<u64>>,
    pub polygon_type_indices: Vec<u64>,
}

impl From<NavigationMesh<XYZ>> for ExternalNavMesh {
    fn from(nav_mesh: NavigationMesh<XYZ>) -> Self {
        let mut vertices = Vec::with_capacity(nav_mesh.vertices.len());
        for v in nav_mesh.vertices {
            vertices.push(Vec3 {
                x: v.x,
                y: v.y,
                z: v.z,
            });
        }

        let mut polygons = Vec::with_capacity(nav_mesh.polygons.len());
        for poly in nav_mesh.polygons {
            let mut new_poly = Vec::with_capacity(poly.len());
            for idx in poly {
                new_poly.push(idx as u64);
            }
            polygons.push(new_poly);
        }

        let mut polygon_type_indices = Vec::with_capacity(nav_mesh.polygon_type_indices.len());
        for idx in nav_mesh.polygon_type_indices {
            polygon_type_indices.push(idx as u64);
        }

        ExternalNavMesh {
            translation: Vec3::ZERO,
            rotation: 0.0,
            vertices,
            polygons,
            polygon_type_indices,
        }
    }
}

impl From<ExternalNavMesh> for NavigationMesh<XYZ> {
    fn from(st_nav_mesh: ExternalNavMesh) -> Self {
        let polygons = st_nav_mesh
            .polygons
            .into_iter()
            .map(|poly| poly.into_iter().map(|idx| idx as usize).collect())
            .collect();

        let polygon_type_indices = st_nav_mesh
            .polygon_type_indices
            .into_iter()
            .map(|idx| idx as usize)
            .collect();

        NavigationMesh {
            vertices: st_nav_mesh.vertices,
            polygons,
            polygon_type_indices,
            height_mesh: None,
        }
    }
}

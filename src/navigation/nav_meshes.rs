use std::collections::HashMap;

use bon::Builder;
use landmass::NavigationMesh;
use serde::Deserialize;
use spacetimedb::{ReducerContext, Table, table};

use crate::{math::Vec3, navigation::coordinates::XYZ, utils::WorldEntity, world::WorldId};

pub type NavMeshId = u64;

#[table(name = steng_nav_mesh)]
#[derive(Clone, Debug, Builder, Deserialize)]
pub struct NavMesh {
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    pub id: NavMeshId,
    #[index(btree)]
    #[builder(default = 1)]
    pub world_id: WorldId,
    #[builder(default = Vec3::ZERO)]
    /// The translation to apply.
    pub translation: Vec3,
    #[builder(default = 0.0)]
    /// The rotation to apply around the "up" direction. Specifically, the up
    /// direction is perpendicular to the plane of movement.
    pub rotation: f32,
    pub vertices: Vec<Vec3>,
    pub polygons: Vec<Vec<u64>>,
    pub polygon_type_indices: Vec<u64>,
}

impl NavMesh {
    pub fn insert(self, ctx: &spacetimedb::ReducerContext) -> Self {
        ctx.db.steng_nav_mesh().insert(self)
    }
}

impl WorldEntity for NavMesh {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_nav_mesh().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_nav_mesh().id().find(id)
    }

    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> HashMap<u64, Self> {
        ctx.db
            .steng_nav_mesh()
            .world_id()
            .filter(world_id)
            .map(|nav_mesh| (nav_mesh.id, nav_mesh))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db
            .steng_nav_mesh()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_nav_mesh().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_nav_mesh().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        for nav_mesh in ctx.db.steng_nav_mesh().world_id().filter(world_id) {
            nav_mesh.delete(ctx);
        }
    }
}

impl From<NavigationMesh<XYZ>> for NavMesh {
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

        NavMesh {
            id: 0,
            world_id: 0,
            translation: Vec3::ZERO,
            rotation: 0.0,
            vertices,
            polygons,
            polygon_type_indices,
        }
    }
}

impl From<NavMesh> for NavigationMesh<XYZ> {
    fn from(st_nav_mesh: NavMesh) -> Self {
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

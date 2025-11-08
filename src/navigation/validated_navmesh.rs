use std::collections::HashMap;

use crate::{math::Vec3, navigation::coordinates::XYZ, utils::WorldEntity, world::WorldId};
use landmass::{
    BoundingBox, Connectivity as LmConnectivity, MeshEdgeRef, ValidNavigationMesh,
    ValidPolygon as LmValidPolygon, Vec3 as LmVec3,
};
use spacetimedb::{ReducerContext, SpacetimeType, Table, table};

#[derive(SpacetimeType)]
pub struct ValidPolygon {
    pub vertices: Vec<u64>,
    pub polygon_index: Vec<Option<u64>>,
    pub reverse_edge: Vec<Option<u64>>,
    pub region: u64,
    pub type_index: u64,
    pub is_bb_empty: bool,
    pub bb_min: Option<Vec3>,
    pub bb_max: Option<Vec3>,
    pub center: Vec3,
}

#[table(name = steng_navmesh)]
pub struct NavMesh {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub world_id: WorldId,
    pub is_bb_empty: bool,
    pub bb_min: Option<Vec3>,
    pub bb_max: Option<Vec3>,
    pub vertices: Vec<Vec3>,
    pub boundary_edge_polygon: Vec<u64>,
    pub boundary_edge_edge: Vec<u64>,
    pub polygons: Vec<ValidPolygon>,
    pub translation: Vec3,
    pub rotation: f32,
}

impl WorldEntity for NavMesh {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navmesh().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_navmesh().id().find(id)
    }

    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> HashMap<u64, Self> {
        ctx.db
            .steng_navmesh()
            .world_id()
            .filter(world_id)
            .map(|nav_mesh| (nav_mesh.id, nav_mesh))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db.steng_navmesh().world_id().filter(world_id).collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navmesh().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_navmesh().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        for nav_mesh in ctx.db.steng_navmesh().world_id().filter(world_id) {
            nav_mesh.delete(ctx);
        }
    }
}

impl From<ValidNavigationMesh<XYZ>> for NavMesh {
    fn from(value: ValidNavigationMesh<XYZ>) -> Self {
        let (is_bb_empty, bb_min, bb_max) = match value.mesh_bounds {
            BoundingBox::Empty => (true, None, None),
            BoundingBox::Box { min, max } => (
                false,
                Some(Vec3::new(min.x, min.y, min.z)),
                Some(Vec3::new(max.x, max.y, max.z)),
            ),
        };

        let vertices = value
            .vertices
            .into_iter()
            .map(|v| Vec3::new(v.x, v.y, v.z))
            .collect();

        let boundary_edge_polygon: Vec<u64> = value
            .boundary_edges
            .iter()
            .map(|be| be.polygon_index as u64)
            .collect();

        let boundary_edge_edge: Vec<u64> = value
            .boundary_edges
            .iter()
            .map(|be| be.edge_index as u64)
            .collect();

        let polygons = value
            .polygons
            .iter()
            .map(|p| {
                let (polygon_index, reverse_edge): (Vec<Option<u64>>, Vec<Option<u64>>) = p
                    .connectivity
                    .iter()
                    .map(|ci| match ci {
                        Some(connectivity) => (
                            Some(connectivity.polygon_index as u64),
                            Some(connectivity.reverse_edge as u64),
                        ),
                        None => (None, None),
                    })
                    .unzip();

                let (is_bb_empty, bb_min, bb_max) = match p.bounds {
                    BoundingBox::Empty => (true, None, None),
                    BoundingBox::Box { min, max } => (
                        false,
                        Some(Vec3::new(min.x, min.y, min.z)),
                        Some(Vec3::new(max.x, max.y, max.z)),
                    ),
                };

                ValidPolygon {
                    vertices: p.vertices.iter().map(|&v| v as u64).collect(),
                    polygon_index,
                    reverse_edge,
                    region: p.region as u64,
                    type_index: p.type_index as u64,
                    is_bb_empty,
                    bb_min,
                    bb_max,
                    center: Vec3::new(p.center.x, p.center.y, p.center.z),
                }
            })
            .collect::<Vec<ValidPolygon>>();

        Self {
            id: 0,
            world_id: 0,
            is_bb_empty,
            bb_min,
            bb_max,
            vertices,
            boundary_edge_polygon,
            boundary_edge_edge,
            polygons,
            translation: Vec3::ZERO,
            rotation: 0.0,
        }
    }
}

impl From<NavMesh> for ValidNavigationMesh<XYZ> {
    fn from(value: NavMesh) -> Self {
        let mesh_bounds = if value.is_bb_empty {
            BoundingBox::Empty
        } else {
            BoundingBox::Box {
                min: LmVec3 {
                    x: value.bb_min.unwrap().x,
                    y: value.bb_min.unwrap().y,
                    z: value.bb_min.unwrap().z,
                },
                max: LmVec3 {
                    x: value.bb_max.unwrap().x,
                    y: value.bb_max.unwrap().y,
                    z: value.bb_max.unwrap().z,
                },
            }
        };

        let vertices = value
            .vertices
            .into_iter()
            .map(|v| LmVec3 {
                x: v.x,
                y: v.y,
                z: v.z,
            })
            .collect();

        let boundary_edges = value
            .boundary_edge_polygon
            .into_iter()
            .zip(value.boundary_edge_edge.iter())
            .map(|(polygon_index, edge_index)| MeshEdgeRef {
                polygon_index: polygon_index as usize,
                edge_index: *edge_index as usize,
            })
            .collect();

        let polygons = value
            .polygons
            .into_iter()
            .map(|p| LmValidPolygon {
                vertices: p.vertices.into_iter().map(|v| v as usize).collect(),
                connectivity: p
                    .polygon_index
                    .into_iter()
                    .zip(p.reverse_edge.iter())
                    .map(|(pi, re)| match (pi, re) {
                        (Some(polygon_index), Some(reverse_edge)) => Some(LmConnectivity {
                            polygon_index: polygon_index as usize,
                            reverse_edge: *reverse_edge as usize,
                        }),
                        _ => None,
                    })
                    .collect(),
                region: p.region as usize,
                type_index: p.type_index as usize,
                center: LmVec3 {
                    x: p.center.x,
                    y: p.center.y,
                    z: p.center.z,
                },
                bounds: if p.is_bb_empty {
                    BoundingBox::Empty
                } else {
                    BoundingBox::Box {
                        min: LmVec3 {
                            x: p.bb_min.unwrap().x,
                            y: p.bb_min.unwrap().y,
                            z: p.bb_min.unwrap().z,
                        },
                        max: LmVec3 {
                            x: p.bb_max.unwrap().x,
                            y: p.bb_max.unwrap().y,
                            z: p.bb_max.unwrap().z,
                        },
                    }
                },
            })
            .collect();

        Self {
            mesh_bounds,
            vertices,
            boundary_edges,
            polygons,
            height_mesh: None,
            marker: std::marker::PhantomData,
        }
    }
}

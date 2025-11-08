use crate::{
    math::Vec3,
    navigation::{NavMesh, coordinates::XYZ},
};
use landmass::NavigationMesh;

pub fn convert_godot_navmesh_to_landmass(nav: NavMesh) -> NavigationMesh<XYZ> {
    let vertices: Vec<Vec3> = nav.vertices;

    let polygons: Vec<Vec<usize>> = nav
        .polygons
        .iter()
        .map(|poly| poly.iter().map(|i| *i as usize).collect())
        .collect();
    let polygon_count = polygons.len();

    NavigationMesh {
        vertices,
        polygons,
        polygon_type_indices: vec![0; polygon_count],
        height_mesh: None,
    }
}

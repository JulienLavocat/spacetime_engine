use crate::{
    math::Vec3,
    navigation::{NavMesh, coordinates::XYZ}, // XYZ must have FLIP_POLYGONS = false
};
use landmass::{HeightNavigationMesh, HeightPolygon, NavigationMesh};

/// Convert a Godot-style `NavMesh` (Y-up) into a Landmass `NavigationMesh<XYZ>` (your Y-up coord system).
/// - Work entirely in Godot space (Y-up): rotate around Y, then translate.
/// - Do NOT swap axes here; `XYZ::to_landmass` will map (x, y, z) -> (x, z, y).
/// - Keep polygon index order as exported by Godot; no manual flipping.
/// - Height mesh uses standard CCW fan [0, k, k+1] in the same space.
pub fn convert_godot_navmesh_to_landmass(nav: NavMesh) -> NavigationMesh<XYZ> {
    // Rotate around Godot up-axis (Y).
    let (sin_r, cos_r) = nav.rotation.sin_cos();

    // 1) Transform vertices in Godot space.
    let vertices: Vec<Vec3> = nav
        .vertices
        .iter()
        .map(|v| {
            // rotate around Y
            let rx = cos_r * v.x + sin_r * v.z;
            let rz = -sin_r * v.x + cos_r * v.z;
            // translate
            Vec3 {
                x: rx + nav.translation.x,
                y: v.y + nav.translation.y,
                z: rz + nav.translation.z,
            }
        })
        .collect();

    // 2) Polygons: keep original order.
    let polygons: Vec<Vec<usize>> = nav
        .polygons
        .iter()
        .map(|poly| poly.iter().map(|i| *i as usize).collect())
        .collect();

    // 3) Height mesh: per polygon, pack vertices contiguously and fan-triangulate CCW.
    let mut h_vertices: Vec<Vec3> = Vec::new();
    let mut h_triangles: Vec<[u8; 3]> = Vec::new();
    let mut h_polys: Vec<HeightPolygon> = Vec::with_capacity(polygons.len());

    let mut base_vertex_index: u32 = 0;
    let mut base_triangle_index: u32 = 0;

    for poly in &polygons {
        let n = poly.len();
        if n < 3 {
            h_polys.push(HeightPolygon {
                base_vertex_index,
                vertex_count: 0,
                base_triangle_index,
                triangle_count: 0,
            });
            continue;
        }

        debug_assert!(n <= u8::MAX as usize, "polygon has {} verts (>255)", n);

        for &vid in poly {
            h_vertices.push(vertices[vid]);
        }

        let mut tri_count = 0u32;
        for k in 1..(n - 1) {
            // CCW fan in Godot space; no flips later because FLIP_POLYGONS = false.
            h_triangles.push([0, k as u8, (k as u8) + 1]);
            tri_count += 1;
        }

        h_polys.push(HeightPolygon {
            base_vertex_index,
            vertex_count: n as u32,
            base_triangle_index,
            triangle_count: tri_count,
        });

        base_vertex_index += n as u32;
        base_triangle_index += tri_count;
    }

    // 4) Polygon type indices.
    let polygon_type_indices: Vec<usize> = if nav.polygon_type_indices.is_empty() {
        vec![0; polygons.len()]
    } else {
        nav.polygon_type_indices
            .into_iter()
            .map(|i| i as usize)
            .collect()
    };

    NavigationMesh {
        vertices,
        polygons,
        polygon_type_indices,
        height_mesh: Some(HeightNavigationMesh {
            polygons: h_polys,
            triangles: h_triangles,
            vertices: h_vertices,
        }),
    }
}

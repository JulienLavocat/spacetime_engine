use crate::{
    navigation::{ExternalNavMesh, coordinates::XYZ, validated_navmesh::NavMesh},
    utils::WorldEntity,
    world::WorldId,
};
use landmass::NavigationMesh;
use spacetimedb::ReducerContext;

pub fn import_external_navmesh(
    ctx: &ReducerContext,
    world_id: WorldId,
    exteranal_navmesh: ExternalNavMesh,
) {
    let translation = exteranal_navmesh.translation;
    let rotation = exteranal_navmesh.rotation;

    let lm_nav_mesh: NavigationMesh<XYZ> = exteranal_navmesh.into();
    let validated_navmesh = lm_nav_mesh.validate().expect("Failed to validate navmesh");

    let encoded = bincode::encode_to_vec(validated_navmesh, bincode::config::standard());

    let mut navmesh: NavMesh = validated_navmesh.into();
    navmesh.translation = translation;
    navmesh.rotation = rotation;
    navmesh.world_id = world_id;

    navmesh.insert(ctx);
}

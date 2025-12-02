use std::collections::HashMap;

use parry3d::{
    na::Isometry3,
    partitioning::{Bvh, BvhBuildStrategy, TraversalAction},
    query::Ray,
};
use spacetimedb::ReducerContext;

use crate::{
    collisions::{
        Collider, ColliderId, RayCast, RayCastHit, RayCastId, RigidBody, RigidBodyId, Trigger,
        TriggerId, shape_wrapper::ShapeWrapper,
    },
    utils::{LogStopwatch, WorldEntity},
    world::World,
};

pub fn tick_collisions(ctx: &ReducerContext, world: &World) {
    let mut sw = LogStopwatch::new(
        ctx,
        world,
        "collisions_tick".to_string(),
        world.debug_collisions,
    );

    sw.span("gather_entities");
    let colliders: HashMap<ColliderId, ShapeWrapper> = Collider::iter(ctx, world.id)
        .map(|collider| {
            let shape = ShapeWrapper::from(&collider);
            (collider.id, shape)
        })
        .collect();
    let rigid_bodies = RigidBody::as_vec(ctx, world.id);
    let mut triggers = Trigger::as_map(ctx, world.id);
    let mut raycasts = RayCast::as_map(ctx, world.id);

    sw.span("broad_phase");
    let (broad_raycast_hits, broad_trigger_hits) =
        run_broad_phase(&rigid_bodies, &colliders, &raycasts, &triggers, world);

    sw.span("narrow_phase");
    let (narrow_raycast_hits, narrow_trigger_hits) = run_narrow_phase(
        broad_raycast_hits,
        broad_trigger_hits,
        &colliders,
        &RigidBody::as_map(ctx, world.id),
        &raycasts,
        &triggers,
    );

    sw.span("update_entities");
    update(
        ctx,
        world,
        &narrow_raycast_hits,
        &narrow_trigger_hits,
        &mut raycasts,
        &mut triggers,
    );
    sw.end();
}

fn run_broad_phase(
    rigid_bodies: &[RigidBody],
    colliders: &HashMap<ColliderId, ShapeWrapper>,
    raycasts: &HashMap<u64, RayCast>,
    triggers: &HashMap<u64, Trigger>,
    world: &World,
) -> (
    HashMap<RayCastId, Vec<RigidBodyId>>,
    HashMap<TriggerId, Vec<RigidBodyId>>,
) {
    let mut aabbs = Vec::with_capacity(rigid_bodies.len());
    let mut body_ids = Vec::with_capacity(rigid_bodies.len());

    for rb in rigid_bodies {
        let collider = colliders.get(&rb.collider_id).unwrap();
        let position = rb.position.into();
        let rotation = rb.rotation.into();
        let aabb = collider.collision_aabb(
            &Isometry3::from_parts(position, rotation),
            world.aabb_dilation_factor,
        );

        aabbs.push(aabb);
        body_ids.push(rb.id);
    }

    let bvh = Bvh::from_leaves(BvhBuildStrategy::Binned, &aabbs);

    let mut raycast_hits: HashMap<RayCastId, Vec<RigidBodyId>> = HashMap::new();
    for raycast in raycasts.values() {
        let ray = Ray::new(raycast.origin.into(), raycast.direction.into());
        let mut hits = Vec::new();

        bvh.traverse(|node| {
            if node.cast_ray(&ray, raycast.max_distance) <= raycast.max_distance {
                if node.is_leaf() {
                    let leaf_idx = node.leaf_data().unwrap() as usize;
                    let rb_id = body_ids[leaf_idx];
                    hits.push(rb_id);
                }
                TraversalAction::Continue
            } else {
                TraversalAction::Prune
            }
        });

        raycast_hits.insert(raycast.id, hits);
    }

    let mut trigger_hits: HashMap<TriggerId, Vec<RigidBodyId>> = HashMap::new();
    for trigger in triggers.values() {
        let collider = colliders.get(&trigger.collider_id).unwrap();
        let aabb = collider.collision_aabb(&Isometry3::identity(), world.aabb_dilation_factor);

        let hits = bvh
            .intersect_aabb(&aabb)
            .map(|leaf_idx| {
                let idx = leaf_idx as usize;
                body_ids[idx]
            })
            .collect::<Vec<_>>();

        trigger_hits.insert(trigger.id, hits);
    }

    (raycast_hits, trigger_hits)
}

fn run_narrow_phase(
    broad_raycast_hits: HashMap<RayCastId, Vec<RigidBodyId>>,
    broad_trigger_hits: HashMap<TriggerId, Vec<RigidBodyId>>,
    colliders: &HashMap<ColliderId, ShapeWrapper>,
    rigid_bodies: &HashMap<RigidBodyId, RigidBody>,
    raycasts: &HashMap<RayCastId, RayCast>,
    triggers: &HashMap<TriggerId, Trigger>,
) -> (
    HashMap<RayCastId, Vec<RayCastHit>>,
    HashMap<TriggerId, Vec<RigidBodyId>>,
) {
    let mut narrow_raycast_hits: HashMap<RayCastId, Vec<RayCastHit>> = HashMap::new();
    for (raycast_id, hits) in broad_raycast_hits {
        let raycast = raycasts.get(&raycast_id).unwrap();
        let ray = Ray::new(raycast.origin.into(), raycast.direction.into());
        let mut valid_hits = Vec::new();
        for rigid_body_id in hits {
            let rigid_body = rigid_bodies.get(&rigid_body_id).unwrap();
            let rigid_body_collider = colliders.get(&rigid_body.collider_id).unwrap();
            let isometry =
                Isometry3::from_parts(rigid_body.position.into(), rigid_body.rotation.into());
            if let Some(hit) = rigid_body_collider.cast_ray_and_get_normal(
                &isometry,
                &ray,
                raycast.max_distance,
                true,
            ) {
                valid_hits.push(RayCastHit {
                    rigid_body_id,
                    position: ray.point_at(hit.time_of_impact).into(),
                    normal: hit.normal.into(),
                    distance: hit.time_of_impact,
                });
            }
        }
        narrow_raycast_hits.insert(raycast_id, valid_hits);
    }

    let mut narrow_trigger_hits: HashMap<TriggerId, Vec<RigidBodyId>> = HashMap::new();
    for (trigger_id, hits) in broad_trigger_hits {
        let trigger = triggers.get(&trigger_id).unwrap();
        let trigger_collider = colliders.get(&trigger.collider_id).unwrap();
        let mut valid_hits = Vec::new();
        for rigid_body_id in hits {
            let rigid_body = rigid_bodies.get(&rigid_body_id).unwrap();
            let rigid_body_collider = colliders.get(&rigid_body.collider_id).unwrap();
            let trigger_isometry =
                Isometry3::from_parts(trigger.position.into(), trigger.rotation.into());
            let rigid_body_isometry =
                Isometry3::from_parts(rigid_body.position.into(), rigid_body.rotation.into());
            if trigger_collider.intersects(
                &trigger_isometry,
                &rigid_body_isometry,
                rigid_body_collider,
            ) {
                valid_hits.push(rigid_body_id);
            }
        }
        narrow_trigger_hits.insert(trigger_id, valid_hits);
    }

    (narrow_raycast_hits, narrow_trigger_hits)
}

fn update(
    ctx: &ReducerContext,
    world: &World,
    raycast_hits: &HashMap<RayCastId, Vec<RayCastHit>>,
    trigger_hits: &HashMap<TriggerId, Vec<RigidBodyId>>,
    raycasts: &mut HashMap<RayCastId, RayCast>,
    triggers: &mut HashMap<TriggerId, Trigger>,
) {
    for (raycast_id, hits) in raycast_hits {
        let raycast = raycasts.get(raycast_id).unwrap();
        let previous_hits: Vec<RayCastHit> = raycast.hits.clone();
        let current_hits: Vec<RayCastHit> = hits.to_vec();
        let mut raycast = raycasts.remove(raycast_id).unwrap();

        raycast.added_hits = current_hits
            .iter()
            .filter(|hit| !previous_hits.contains(hit))
            .cloned()
            .collect();
        raycast.removed_hits = previous_hits
            .iter()
            .filter(|hit| !current_hits.contains(hit))
            .cloned()
            .collect();
        raycast.hits = hits.to_vec();

        raycast.update(ctx);
    }

    for (trigger_id, hits) in trigger_hits {
        let mut trigger = triggers.remove(trigger_id).unwrap();
        let previous_hits: Vec<RigidBodyId> = trigger.entities_inside.clone();
        let current_hits: Vec<RigidBodyId> = hits.to_vec();

        trigger.added_entities = current_hits
            .iter()
            .filter(|id| !previous_hits.contains(id))
            .cloned()
            .collect();
        trigger.removed_entities = previous_hits
            .iter()
            .filter(|id| !current_hits.contains(id))
            .cloned()
            .collect();
        trigger.entities_inside = hits.to_vec();

        let is_different =
            !trigger.added_entities.is_empty() || !trigger.removed_entities.is_empty();
        if world.debug_collisions && is_different {
            log::debug!(
                "[PhysicsWorld#{}] [Trigger] Trigger#{} entities inside: {:?}, added: {:?}, removed: {:?}",
                world.id,
                trigger.id,
                trigger.entities_inside,
                trigger.added_entities,
                trigger.removed_entities
            );
        }

        trigger.update(ctx);
    }
}

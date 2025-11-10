use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use landmass::{
    Agent, Archipelago as LmArchipelago, ArchipelagoOptions, Island, PointSampleDistance3d,
    Transform, ValidNavigationMesh,
};
use spacetimedb::ReducerContext;

use crate::{
    navigation::{
        NavigationAgent, NavigationAgentId, archipelago::ArchipelagoData, coordinates::XYZ,
        validated_navmesh::NavMesh,
    },
    utils::{LogStopwatch, WorldEntity},
    world::World,
};

pub type Archipelago = LmArchipelago<XYZ>;

pub(crate) fn tick_navigation(
    ctx: &ReducerContext,
    world: World,
    delta_time: f32,
) -> HashMap<NavigationAgentId, NavigationAgent> {
    let mut sw = LogStopwatch::new(ctx, &world, "navigation_tick".to_string());

    let (mut archipelago, existing_data) = retrieve_archipelago(ctx, &world, &mut sw);
    let mut agents_by_id = sync_agents(ctx, &world, &mut archipelago, &mut sw);

    sw.span("update_archipelago");
    archipelago.update(&mut ctx.rng(), delta_time);

    sw.span("update_agents");
    for lm_agent_id in archipelago.get_agent_ids() {
        let lm_agent = archipelago.get_agent(lm_agent_id).unwrap();
        let mut eng_agent = agents_by_id.remove(&lm_agent.external_id).unwrap();

        let velocity = lm_agent.get_desired_velocity();
        let old_position = eng_agent.position;
        let new_pos = eng_agent.position + *velocity * delta_time;
        if let Ok(point) = archipelago.sample_point(
            new_pos,
            &archipelago.archipelago_options.point_sample_distance,
        ) {
            eng_agent.position = point.point();
        }
        eng_agent.state = lm_agent.state().into();
        eng_agent.velocity = *velocity;

        log::debug!(
            "Agent {} moved from {} to {} with velocity {}",
            eng_agent.id,
            old_position,
            eng_agent.position,
            eng_agent.velocity
        );
        eng_agent = eng_agent.update(ctx);
        agents_by_id.insert(eng_agent.id, eng_agent);
    }
    sw.end();

    sw.span("serialize_archipelago");
    let archipelago_data = bincode::serde::encode_to_vec(&archipelago, bincode::config::standard())
        .expect("Failed to serialize archipelago");

    if let Some(mut existing_data) = existing_data {
        existing_data.data = archipelago_data;
        existing_data.update(ctx);
    } else {
        let new_data = ArchipelagoData {
            id: 0,
            world_id: world.id,
            data: archipelago_data,
        };
        new_data.insert(ctx);
    }
    sw.end();

    agents_by_id
}

fn retrieve_archipelago(
    ctx: &ReducerContext,
    world: &World,
    sw: &mut LogStopwatch,
) -> (Archipelago, Option<ArchipelagoData>) {
    if let Some(archipelago_data) = ArchipelagoData::iter(ctx, world.id).next() {
        sw.span("deserialize_archipelago");
        return (
            bincode::serde::decode_from_slice(&archipelago_data.data, bincode::config::standard())
                .expect("Failed to decode archipelago")
                .0,
            Some(archipelago_data),
        );
    }

    sw.span("load_navmeshes");
    let radius = 0.5;
    let mut archipelago = Archipelago::new(ArchipelagoOptions {
        point_sample_distance: PointSampleDistance3d {
            horizontal_distance: radius,
            distance_above: radius * 2.0,
            distance_below: radius * 2.0,
            vertical_preference_ratio: 2.0,
            animation_link_max_vertical_distance: 0.5 * radius,
        },
        neighbourhood: 10.0 * radius,
        avoidance_time_horizon: 0.5,
        obstacle_avoidance_time_horizon: 0.25,
        reached_destination_avoidance_responsibility: 0.1,
    });

    for navmesh in NavMesh::iter(ctx, world.id) {
        let translation = navmesh.translation;
        let rotation = navmesh.rotation;

        let navmesh: ValidNavigationMesh<XYZ> =
            bincode::serde::decode_from_slice(&navmesh.data, bincode::config::standard())
                .expect("Failed to decode navmesh")
                .0;
        let navmesh: Arc<ValidNavigationMesh<XYZ>> = Arc::new(navmesh);

        archipelago.add_island(Island::new(
            Transform {
                translation,
                rotation,
            },
            navmesh,
        ));
    }

    (archipelago, None)
}

fn sync_agents(
    ctx: &ReducerContext,
    world: &World,
    archipelago: &mut Archipelago,
    sw: &mut LogStopwatch,
) -> HashMap<NavigationAgentId, NavigationAgent> {
    sw.span("sync_agents");
    let agents_by_id = NavigationAgent::as_map(ctx, world.id);
    let eng_ids: HashSet<NavigationAgentId> = agents_by_id.keys().copied().collect();

    let mut present_ids: HashSet<NavigationAgentId> = HashSet::with_capacity(eng_ids.len());
    let mut to_remove = Vec::new();

    for (lm_id, lm) in archipelago.get_agents() {
        if eng_ids.contains(&lm.external_id) {
            present_ids.insert(lm.external_id);
        } else {
            to_remove.push(lm_id);
        }
    }

    for lm_id in to_remove {
        archipelago.remove_agent(lm_id);
    }

    for &agent_id in eng_ids.difference(&present_ids) {
        let eng = &agents_by_id[&agent_id];

        let mut lm = Agent::create(
            eng.position,
            eng.velocity,
            eng.radius,
            eng.desired_speed,
            eng.max_speed,
        );
        lm.external_id = agent_id;
        lm.current_target = eng.current_target;
        lm.target_reached_condition = eng.target_reached_condition.into();
        lm.state = eng.state.into();

        archipelago.add_agent(lm);
    }

    agents_by_id
}

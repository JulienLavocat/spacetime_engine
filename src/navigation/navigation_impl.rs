use std::{collections::HashMap, sync::Arc};

use landmass::{
    Agent, Archipelago as LmArchipelago, ArchipelagoOptions, Island, PointSampleDistance3d,
    Transform, ValidNavigationMesh,
};
use spacetimedb::ReducerContext;

use crate::{
    navigation::{
        NavigationAgent, NavigationAgentId, coordinates::XYZ, validated_navmesh::NavMesh,
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

    sw.span("build_archipelago");
    let radius = 0.5;
    // TODO: Curently recreating the archipelago every tick makes computed paths invalid,
    // causing agents to recompute paths every tick.
    // We need to persist the archipelago between ticks.
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

    sw.span("create_agents");
    let mut agents = HashMap::new();

    for eng_agent in NavigationAgent::iter(ctx, world.id) {
        let mut lm = Agent::create(
            eng_agent.position,
            eng_agent.velocity,
            eng_agent.radius,
            eng_agent.desired_speed,
            eng_agent.max_speed,
        );
        lm.current_target = eng_agent.current_target;
        lm.target_reached_condition = eng_agent.target_reached_condition.into();
        lm.state = eng_agent.state.into();

        let agent_id = archipelago.add_agent(lm);
        agents.insert(agent_id, eng_agent);
    }

    sw.span("update_archipelago");
    archipelago.update(&mut ctx.rng(), delta_time);

    sw.span("update_agents");
    let mut updated_agents = HashMap::new();

    for (lm_agent_id, mut eng_agent) in agents {
        let lm_agent = archipelago.get_agent(lm_agent_id).unwrap();
        let velocity = lm_agent.get_desired_velocity();

        eng_agent.velocity = *velocity;
        let new_pos = eng_agent.position + velocity * delta_time;
        if let Ok(point) = archipelago.sample_point(
            new_pos,
            &archipelago.archipelago_options.point_sample_distance,
        ) {
            eng_agent.position = point.point();
        }

        eng_agent.state = lm_agent.state().into();
        let navagent = eng_agent.update(ctx);
        updated_agents.insert(navagent.id, navagent);
    }
    sw.end();

    updated_agents
}

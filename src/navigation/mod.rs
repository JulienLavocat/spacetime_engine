use std::{collections::HashMap, sync::Arc};

use bon::Builder;
use landmass::{
    Agent, AgentId, Archipelago, ArchipelagoOptions, FromAgentRadius, Island, NavigationMesh,
    Transform,
};
use spacetimedb::{ReducerContext, Table, table};

use crate::{
    math::Vec3,
    navigation::coordinates::XYZ,
    utils::LogStopwatch,
    world::{World, WorldId},
};

mod agent_state;
mod coordinates;
mod nav_meshes;
mod target_reached_condition;
mod utils;

pub use agent_state::*;
pub use nav_meshes::*;
pub use target_reached_condition::*;
pub use utils::*;

pub type NavigationAgentId = u64;

#[table(name = steng_navigation_agent)]
#[derive(Clone, Debug, Builder)]
pub struct NavigationAgent {
    /// The unique ID of the agent. This ID is unique across all worlds.
    #[primary_key]
    #[auto_inc]
    #[builder(default = 0)]
    pub id: NavigationAgentId,
    /// The ID of the world this agent belongs to.
    #[index(btree)]
    #[builder(default = 1)]
    pub world_id: WorldId,
    /// An optional external ID for the agent. This can be used to link the
    /// agent to an entity in another system.
    pub external_id: Option<u64>,
    /// The current position of the agent.
    #[builder(default = Vec3::ZERO)]
    pub position: Vec3,
    /// The current velocity of the agent.
    #[builder(default = Vec3::ZERO)]
    pub velocity: Vec3,
    /// The current target to move towards. Modifying this every update is fine.
    /// Paths will be reused for target points near each other if possible.
    /// However, swapping between two distant targets every update can be
    /// detrimental to be performance.
    pub current_target: Option<Vec3>,
    /// The state of the agent.
    #[builder(default = AgentState::Idle)]
    pub state: AgentState,
    /// The condition to test for reaching the target.
    #[builder(default = TargetReachedCondition::Distance(None))]
    pub target_reached_condition: TargetReachedCondition,
    /// The radius of the agent.
    #[builder(default = 0.5)]
    pub radius: f32,
    /// The speed the agent prefers to move at. This should often be set lower
    /// than the [`Self::max_speed`] to allow the agent to "speed up" in order to
    /// get out of another agent's way.
    #[builder(default = 1.0)]
    pub desired_speed: f32,
    /// The maximum speed that the agent can move at.
    #[builder(default = 2.0)]
    pub max_speed: f32,
    /// Whether this agent is "paused". Paused agents are not considered for
    /// avoidance, and will not recompute their paths. However, their paths are
    /// still kept "consistent" - meaning that once the agent becomes unpaused,
    /// it can reuse that path if it is still valid and relevant (the agent still
    /// wants to go to the same place).
    #[builder(default = false)]
    pub paused: bool,
}

impl NavigationAgent {
    pub fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navigation_agent().insert(self)
    }

    pub fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navigation_agent().id().update(self)
    }

    pub fn find(ctx: &ReducerContext, id: NavigationAgentId) -> Option<Self> {
        ctx.db.steng_navigation_agent().id().find(id)
    }

    pub fn delete(self, ctx: &ReducerContext) {
        ctx.db.steng_navigation_agent().id().delete(self.id);
    }
}

pub(crate) fn tick_navigation(
    ctx: &ReducerContext,
    world: World,
    delta_time: f32,
) -> HashMap<NavigationAgentId, NavigationAgent> {
    let mut archipelago = Archipelago::<XYZ>::new(ArchipelagoOptions::from_agent_radius(1.0));

    let rebuild_sw = LogStopwatch::new(&world, "REBUILD_ARCHIPELAGO");
    for navmesh in ctx.db.steng_nav_mesh().world_id().filter(world.id) {
        let translation = navmesh.translation;
        let rotation = navmesh.rotation;
        let navmesh: NavigationMesh<XYZ> = navmesh.into();

        // TODO: Find a way to remove this validation step on every tick,
        // perhaps by storing the validated navmesh in the database.
        let valid_navmesh = Arc::new(navmesh.validate().expect("Invalid navmesh"));

        archipelago.add_island(Island::new(
            Transform {
                translation,
                rotation,
            },
            valid_navmesh,
        ));
    }

    let agents: Vec<(AgentId, NavigationAgent)> = ctx
        .db
        .steng_navigation_agent()
        .world_id()
        .filter(world.id)
        .map(|navagent| {
            let mut agent = Agent::create(
                navagent.position,
                navagent.velocity,
                navagent.radius,
                navagent.desired_speed,
                navagent.max_speed,
            );
            agent.current_target = navagent.current_target;
            agent.target_reached_condition = navagent.target_reached_condition.into();
            let agent_id = archipelago.add_agent(agent);

            (agent_id, navagent)
        })
        .collect();

    rebuild_sw.end();

    // In principe re-creating the archipelago every tick is fine but
    // we should save the path computed for each agent to avoid
    // recomputing them every tick.
    let substep_delta_time = delta_time / world.navigation_substeps as f32;
    let update_sw = LogStopwatch::new(&world, "UPDATE_ARCHIPELAGO");
    archipelago.update(&mut ctx.rng(), substep_delta_time);
    update_sw.end();

    let update_sw = LogStopwatch::new(&world, "UPDATE_AGENTS");
    let mut updated_agents = Vec::with_capacity(agents.len());

    for (agent_id, mut navagent) in agents {
        let agent = archipelago.get_agent(agent_id).unwrap();
        let velocity = agent.get_desired_velocity().into();

        navagent.position += velocity * delta_time;
        navagent.velocity = velocity;
        navagent.state = agent.state().into();

        let agent = ctx.db.steng_navigation_agent().id().update(navagent);
        updated_agents.push(agent);
    }
    update_sw.end();

    updated_agents.into_iter().map(|a| (a.id, a)).collect()
}

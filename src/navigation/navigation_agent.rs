use std::collections::HashMap;

use bon::Builder;
use spacetimedb::{ReducerContext, Table, table};

use crate::{
    math::Vec3,
    navigation::{AgentState, TargetReachedCondition},
    utils::WorldEntity,
    world::WorldId,
};

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

impl WorldEntity for NavigationAgent {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navigation_agent().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_navigation_agent().id().find(id)
    }

    fn as_map(ctx: &ReducerContext, world_id: WorldId) -> HashMap<u64, Self> {
        ctx.db
            .steng_navigation_agent()
            .world_id()
            .filter(world_id)
            .map(|agent| (agent.id, agent))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext, world_id: WorldId) -> Vec<Self> {
        ctx.db
            .steng_navigation_agent()
            .world_id()
            .filter(world_id)
            .collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.steng_navigation_agent().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.steng_navigation_agent().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext, world_id: WorldId) {
        for agent in ctx.db.steng_navigation_agent().world_id().filter(world_id) {
            ctx.db.steng_navigation_agent().id().delete(agent.id);
        }
    }

    fn iter(ctx: &ReducerContext, world_id: WorldId) -> impl Iterator<Item = Self> {
        ctx.db.steng_navigation_agent().world_id().filter(world_id)
    }
}

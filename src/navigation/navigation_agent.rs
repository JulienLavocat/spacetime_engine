use std::collections::HashMap;

use bon::Builder;
use landmass::Agent;
use spacetimedb::{ReducerContext, Table, table};

use crate::{
    math::Vec3,
    navigation::{DestinationReachedCondition, NavigationState, coordinates::XYZ},
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
    id: NavigationAgentId,
    /// The ID of the world this agent belongs to.
    #[index(btree)]
    #[builder(default = 1)]
    world_id: WorldId,
    /// An optional external ID for the agent. This can be used to link the
    /// agent to an entity in another system.
    pub external_id: Option<u64>,
    /// The current position of the agent.
    #[builder(default = Vec3::ZERO)]
    position: Vec3,
    /// The current velocity of the agent.
    #[builder(default = Vec3::ZERO)]
    velocity: Vec3,
    /// The current target to move towards. Modifying this every update is fine.
    /// Paths will be reused for target points near each other if possible.
    /// However, swapping between two distant targets every update can be
    /// detrimental to be performance.
    current_target: Option<Vec3>,
    /// The state of the agent.
    #[builder(default = NavigationState::Idle)]
    state: NavigationState,
    /// The condition to test for reaching the target.
    #[builder(default = DestinationReachedCondition::Distance(None))]
    target_reached_condition: DestinationReachedCondition,
    /// The radius of the agent.
    #[builder(default = 0.5)]
    radius: f32,
    /// The speed the agent prefers to move at. This should often be set lower
    /// than the [`Self::max_speed`] to allow the agent to "speed up" in order to
    /// get out of another agent's way.
    #[builder(default = 1.0)]
    desired_speed: f32,
    /// The maximum speed that the agent can move at.
    #[builder(default = 2.0)]
    max_speed: f32,
    /// Whether this agent is "paused". Paused agents are not considered for
    /// avoidance, and will not recompute their paths. However, their paths are
    /// still kept "consistent" - meaning that once the agent becomes unpaused,
    /// it can reuse that path if it is still valid and relevant (the agent still
    /// wants to go to the same place).
    #[builder(default = false)]
    paused: bool,
}

impl NavigationAgent {
    /// Returns true if the agent is currently moving.
    pub fn is_moving(&self) -> bool {
        matches!(self.state, NavigationState::Moving)
    }

    /// Returns true if the agent is currently idle (has no target).
    pub fn is_idle(&self) -> bool {
        matches!(self.state, NavigationState::Idle)
    }

    /// Returns true if the agent is currently paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Returns true if the agent has reached its target.
    pub fn has_reached_destination(&self) -> bool {
        matches!(self.state, NavigationState::ReachedDestination)
    }

    /// The current position to move towards.
    pub fn destination(&self) -> Option<Vec3> {
        self.current_target
    }

    /// The current speed of the agent.
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    /// The state of the agent.
    pub fn state(&self) -> NavigationState {
        self.state
    }

    pub fn id(&self) -> NavigationAgentId {
        self.id
    }

    /// The ID of the world this agent belongs to.
    pub fn world_id(&self) -> WorldId {
        self.world_id
    }

    /// The current position of the agent.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// The current velocity of the agent.
    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    /// The radius of the agent.
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// The speed the agent prefers to move at. This should often be set lower
    /// than the [`Self::max_speed`] to allow the agent to "speed up" in order to
    /// get out of another agent's way.
    pub fn desired_speed(&self) -> f32 {
        self.desired_speed
    }

    /// The maximum speed that the agent can move at.
    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }

    /// Sets whether this agent is "paused". Paused agents are not considered for
    /// avoidance, and will not recompute their paths. However, their paths are
    /// still kept "consistent" - meaning that once the agent becomes unpaused,
    /// it can reuse that path if it is still valid and relevant (the agent still
    /// wants to go to the same place).
    pub fn set_paused(&mut self, paused: bool) -> &mut Self {
        self.paused = paused;
        self
    }

    /// The current position to move towards. Modifying this every update is fine.
    /// Paths will be reused for points near each other if possible.
    /// However, swapping between two distant position every update can be
    /// detrimental to be performance.
    pub fn set_destination(&mut self, target: Option<Vec3>) -> &mut Self {
        self.current_target = target;
        self
    }

    pub fn set_destination_reached_condition(
        &mut self,
        condition: DestinationReachedCondition,
    ) -> &mut Self {
        self.target_reached_condition = condition;
        self
    }

    pub fn set_desired_speed(&mut self, speed: f32) -> &mut Self {
        self.desired_speed = speed;
        self
    }

    pub fn set_max_speed(&mut self, speed: f32) -> &mut Self {
        self.max_speed = speed;
        self
    }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_position(&mut self, position: Vec3) -> &mut Self {
        self.position = position;
        self
    }

    pub fn set_velocity(&mut self, velocity: Vec3) -> &mut Self {
        self.velocity = velocity;
        self
    }

    pub fn set_state(&mut self, state: NavigationState) -> &mut Self {
        self.state = state;
        self
    }
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

    fn count(ctx: &ReducerContext, world_id: WorldId) -> usize {
        ctx.db
            .steng_navigation_agent()
            .world_id()
            .filter(world_id)
            .count()
    }
}

impl From<&NavigationAgent> for landmass::Agent<XYZ> {
    fn from(value: &NavigationAgent) -> Self {
        let mut lm = Agent::create(
            value.position(),
            value.velocity(),
            value.radius(),
            value.desired_speed(),
            value.max_speed(),
        );
        lm.current_target = value.current_target;
        lm.target_reached_condition = value.target_reached_condition.into();
        lm.state = value.state.into();

        lm
    }
}

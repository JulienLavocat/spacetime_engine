use spacetimedb::SpacetimeType;

#[derive(Clone, Copy, PartialEq, Eq, Debug, SpacetimeType)]
pub enum AgentState {
    /// The agent is idle, due to not having a target. Note this does not mean
    /// that they are motionless. An agent will still avoid nearby agents.
    Idle,
    /// The agent has reached their target. The agent may resume moving if the
    /// target moves or otherwise changes.
    ReachedTarget,
    /// The agent has reached an animation link along its path to the target.
    ///
    /// See [`Agent::reached_animation_link`] for details about the link.
    ReachedAnimationLink,
    /// The agent is currently using an animation link.
    UsingAnimationLink,
    /// The agent has a path and is moving towards their target.
    Moving,
    /// The agent is not on a nav mesh.
    AgentNotOnNavMesh,
    /// The target is not on a nav mesh.
    TargetNotOnNavMesh,
    /// The agent has a target but cannot find a path to it.
    NoPath,
    /// The agent is paused.
    Paused,
}

impl From<landmass::AgentState> for AgentState {
    fn from(state: landmass::AgentState) -> Self {
        match state {
            landmass::AgentState::Idle => AgentState::Idle,
            landmass::AgentState::ReachedTarget => AgentState::ReachedTarget,
            landmass::AgentState::ReachedAnimationLink => AgentState::ReachedAnimationLink,
            landmass::AgentState::UsingAnimationLink => AgentState::UsingAnimationLink,
            landmass::AgentState::Moving => AgentState::Moving,
            landmass::AgentState::AgentNotOnNavMesh => AgentState::AgentNotOnNavMesh,
            landmass::AgentState::TargetNotOnNavMesh => AgentState::TargetNotOnNavMesh,
            landmass::AgentState::NoPath => AgentState::NoPath,
            landmass::AgentState::Paused => AgentState::Paused,
        }
    }
}

impl From<AgentState> for landmass::AgentState {
    fn from(state: AgentState) -> Self {
        match state {
            AgentState::Idle => landmass::AgentState::Idle,
            AgentState::ReachedTarget => landmass::AgentState::ReachedTarget,
            AgentState::ReachedAnimationLink => landmass::AgentState::ReachedAnimationLink,
            AgentState::UsingAnimationLink => landmass::AgentState::UsingAnimationLink,
            AgentState::Moving => landmass::AgentState::Moving,
            AgentState::AgentNotOnNavMesh => landmass::AgentState::AgentNotOnNavMesh,
            AgentState::TargetNotOnNavMesh => landmass::AgentState::TargetNotOnNavMesh,
            AgentState::NoPath => landmass::AgentState::NoPath,
            AgentState::Paused => landmass::AgentState::Paused,
        }
    }
}

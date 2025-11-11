use spacetimedb::SpacetimeType;

#[derive(Clone, Copy, PartialEq, Eq, Debug, SpacetimeType)]
pub enum NavigationState {
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

impl From<landmass::AgentState> for NavigationState {
    fn from(state: landmass::AgentState) -> Self {
        match state {
            landmass::AgentState::Idle => NavigationState::Idle,
            landmass::AgentState::ReachedTarget => NavigationState::ReachedTarget,
            landmass::AgentState::ReachedAnimationLink => NavigationState::ReachedAnimationLink,
            landmass::AgentState::UsingAnimationLink => NavigationState::UsingAnimationLink,
            landmass::AgentState::Moving => NavigationState::Moving,
            landmass::AgentState::AgentNotOnNavMesh => NavigationState::AgentNotOnNavMesh,
            landmass::AgentState::TargetNotOnNavMesh => NavigationState::TargetNotOnNavMesh,
            landmass::AgentState::NoPath => NavigationState::NoPath,
            landmass::AgentState::Paused => NavigationState::Paused,
        }
    }
}

impl From<NavigationState> for landmass::AgentState {
    fn from(state: NavigationState) -> Self {
        match state {
            NavigationState::Idle => landmass::AgentState::Idle,
            NavigationState::ReachedTarget => landmass::AgentState::ReachedTarget,
            NavigationState::ReachedAnimationLink => landmass::AgentState::ReachedAnimationLink,
            NavigationState::UsingAnimationLink => landmass::AgentState::UsingAnimationLink,
            NavigationState::Moving => landmass::AgentState::Moving,
            NavigationState::AgentNotOnNavMesh => landmass::AgentState::AgentNotOnNavMesh,
            NavigationState::TargetNotOnNavMesh => landmass::AgentState::TargetNotOnNavMesh,
            NavigationState::NoPath => landmass::AgentState::NoPath,
            NavigationState::Paused => landmass::AgentState::Paused,
        }
    }
}

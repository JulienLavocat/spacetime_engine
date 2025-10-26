use landmass::TargetReachedCondition as LmTargetReachedCondition;
use spacetimedb::SpacetimeType;

/// The condition to consider the agent as having reached its target. When this
/// condition is satisfied, the agent will stop moving.
#[derive(Clone, Copy, Debug, SpacetimeType)]
pub enum TargetReachedCondition {
    /// The target is reached if it is within the provided (Euclidean) distance
    /// of the agent. Useful if the target is surrounded by small obstacles
    /// which don't need to be navigated around (e.g. the agent just needs to
    /// be close enough to shoot at the target, which is surrounded by cover).
    /// Alternatively, if the distance is low, this can simply mean "when the
    /// agent is
    Distance(Option<f32>),

    /// The target is reached if it is "visible" (there is a straight line from
    /// the agent to the target), and the target is within the provided
    /// (Euclidean) distance of the agent. Useful if the agent should be able
    /// to see the target (e.g. a companion character should remain visible to
    /// the player, but should ideally not stand too close). If None, the agent's
    /// radius is used.
    VisibleAtDistance(Option<f32>),
    /// The target is reached if the "straight line" path from the agent to the
    /// target is less than the provided distance. "Straight line" path means if
    /// the agent's path goes around a corner, the distance will be computed
    /// going around the corner. This can be more computationally expensive, as
    /// the straight line path must be computed every update. Useful for agents
    /// that care about the actual walking distance to the target. If None, the
    /// agent's radius is used.
    StraightPathDistance(Option<f32>),
}

impl From<TargetReachedCondition> for LmTargetReachedCondition {
    fn from(condition: TargetReachedCondition) -> Self {
        match condition {
            TargetReachedCondition::Distance(dist) => LmTargetReachedCondition::Distance(dist),
            TargetReachedCondition::VisibleAtDistance(dist) => {
                LmTargetReachedCondition::VisibleAtDistance(dist)
            }
            TargetReachedCondition::StraightPathDistance(dist) => {
                LmTargetReachedCondition::StraightPathDistance(dist)
            }
        }
    }
}

impl From<LmTargetReachedCondition> for TargetReachedCondition {
    fn from(condition: LmTargetReachedCondition) -> Self {
        match condition {
            LmTargetReachedCondition::Distance(dist) => TargetReachedCondition::Distance(dist),
            LmTargetReachedCondition::VisibleAtDistance(dist) => {
                TargetReachedCondition::VisibleAtDistance(dist)
            }
            LmTargetReachedCondition::StraightPathDistance(dist) => {
                TargetReachedCondition::StraightPathDistance(dist)
            }
        }
    }
}
